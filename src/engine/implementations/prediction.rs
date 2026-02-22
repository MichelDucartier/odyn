use tch::{nn, nn::Module, Tensor};

use super::repr::mlp_block;

/// Number of possible moves encoded as (from_square, to_square) = 64 * 64.
const NUM_MOVES: i64 = 64 * 64;

/// Prediction network for MuZero.
///
/// Takes a hidden state (produced by the representation network) and outputs:
/// - A policy: a probability distribution over all possible moves (size 4096).
/// - A value: a scalar evaluation of the position.
pub struct PredictionNet {
    trunk: Box<dyn Module>,
    policy_head: Box<dyn Module>,
    value_head: Box<dyn Module>,
}

impl PredictionNet {
    pub fn new(path: nn::Path, input_dim: i64, hidden_dim: i64) -> Self {
        let trunk = Box::new(nn::seq().add(mlp_block(
            path.clone() / "trunk_mlp1",
            input_dim,
            hidden_dim,
        )));

        let policy_head = Box::new(
            nn::seq()
                .add(mlp_block(
                    path.clone() / "policy_mlp",
                    hidden_dim,
                    hidden_dim,
                ))
                .add(nn::linear(
                    path.clone() / "policy_out",
                    hidden_dim,
                    NUM_MOVES,
                    Default::default(),
                )),
        );

        let value_head = Box::new(
            nn::seq()
                .add(mlp_block(
                    path.clone() / "value_mlp",
                    hidden_dim,
                    hidden_dim,
                ))
                .add(nn::linear(
                    path / "value_out",
                    hidden_dim,
                    1,
                    Default::default(),
                )),
        );

        Self {
            trunk,
            policy_head,
            value_head,
        }
    }

    /// Forward pass.
    ///
    /// Returns `(policy, value)` where:
    /// - `policy` has shape `[batch, 4096]` (or `[4096]` for unbatched input)
    ///    and sums to 1 along the last dimension (softmax).
    /// - `value` has shape `[batch]` (or scalar `[]` for unbatched input)
    ///    in the range `(-1, 1)` (tanh).
    pub fn forward(&self, state: &Tensor) -> (Tensor, Tensor) {
        let unbatched = state.size().len() == 1;

        let x = if unbatched {
            state.unsqueeze(0)
        } else {
            state.shallow_clone()
        };

        let hidden = self.trunk.forward(&x);

        let policy_logits = self.policy_head.forward(&hidden);
        let policy = policy_logits.softmax(-1, policy_logits.kind());

        let value = self.value_head.forward(&hidden).squeeze_dim(-1).tanh();

        if unbatched {
            (policy.squeeze_dim(0), value.squeeze_dim(0))
        } else {
            (policy, value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::START_FEN, engine::implementations::repr::embed_chessboard,
        game::chessboard::Chessboard,
    };
    use tch::{nn, nn::Module, Kind};

    #[test]
    fn test_prediction_net_output_shapes() {
        let input_dim: i64 = 64;
        let hidden_dim: i64 = 32;

        let vs = nn::VarStore::new(tch::Device::Cpu);
        let net = PredictionNet::new(vs.root() / "pred", input_dim, hidden_dim);

        // Unbatched
        let state = Tensor::randn([input_dim], (Kind::Float, tch::Device::Cpu));
        let (policy, value) = net.forward(&state);
        assert_eq!(policy.size(), vec![NUM_MOVES]);
        assert_eq!(value.size(), Vec::<i64>::new()); // scalar

        // Batched
        let batch = Tensor::randn([4, input_dim], (Kind::Float, tch::Device::Cpu));
        let (policy_b, value_b) = net.forward(&batch);
        assert_eq!(policy_b.size(), vec![4, NUM_MOVES]);
        assert_eq!(value_b.size(), vec![4]);
    }

    #[test]
    fn test_repr_then_prediction_end_to_end() {
        let output_dim: i64 = 64;
        let hidden_dim: i64 = 32;

        let board = Chessboard::from_fen(START_FEN, " ");
        let input = board.to_tensor();

        let vs = nn::VarStore::new(input.device());

        // Representation network
        let repr_net = embed_chessboard(vs.root() / "repr", 32, output_dim, 3);
        let state = repr_net.forward(&input); // shape: [output_dim]
        assert_eq!(state.size(), vec![output_dim]);

        // Prediction network (input_dim == output_dim of repr)
        let pred_net = PredictionNet::new(vs.root() / "pred", output_dim, hidden_dim);
        let (policy, value) = pred_net.forward(&state);

        // --- Check output dimensions ---
        assert_eq!(
            policy.size(),
            vec![NUM_MOVES],
            "policy should have 4096 entries"
        );
        assert_eq!(value.size(), Vec::<i64>::new(), "value should be a scalar");

        // --- Check policy is a valid probability distribution ---
        // All entries >= 0
        let min_val: f64 = policy.min().double_value(&[]);
        assert!(
            min_val >= 0.0,
            "policy contains negative values: min = {min_val}"
        );

        // Sums to 1
        let sum: f64 = policy.sum(Kind::Float).double_value(&[]);
        assert!(
            (sum - 1.0).abs() < 1e-5,
            "policy does not sum to 1: sum = {sum}"
        );

        // --- Check value is in (-1, 1) ---
        let v: f64 = value.double_value(&[]);
        assert!(v > -1.0 && v < 1.0, "value should be in (-1, 1), got {v}");
    }
}
