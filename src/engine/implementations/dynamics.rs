use tch::{nn, nn::Module, Tensor};

use crate::constants::NUM_MOVES;

use super::blocks::mlp_block;

/// Dynamics network for MuZero-style rollouts.
///
/// Takes the current hidden state (from the representation network) and a one-hot
/// encoded action vector of size 4096 (64x64), and outputs:
/// - the predicted next hidden state (same dimension as the input state)
/// - the expected reward for that transition
pub struct DynamicsNet {
    trunk: Box<dyn Module>,
    next_state_head: Box<dyn Module>,
    reward_head: Box<dyn Module>,
}

impl DynamicsNet {
    pub fn new(path: nn::Path, state_dim: i64, hidden_dim: i64) -> Self {
        let trunk = Box::new(nn::seq().add(mlp_block(
            path.clone() / "trunk_mlp1",
            state_dim + NUM_MOVES,
            hidden_dim,
        )));

        let next_state_head = Box::new(
            nn::seq()
                .add(mlp_block(
                    path.clone() / "next_state_mlp",
                    hidden_dim,
                    hidden_dim,
                ))
                .add(nn::linear(
                    path.clone() / "next_state_out",
                    hidden_dim,
                    state_dim,
                    Default::default(),
                )),
        );

        let reward_head = Box::new(
            nn::seq()
                .add(mlp_block(
                    path.clone() / "reward_mlp",
                    hidden_dim,
                    hidden_dim,
                ))
                .add(nn::linear(
                    path / "reward_out",
                    hidden_dim,
                    1,
                    Default::default(),
                )),
        );

        Self {
            trunk,
            next_state_head,
            reward_head,
        }
    }

    /// Forward pass.
    ///
    /// `state` and `action` can be unbatched or batched, but both must match:
    /// - unbatched: `state = [state_dim]`, `action = [4096]`
    /// - batched: `state = [batch, state_dim]`, `action = [batch, 4096]`
    ///
    /// Returns `(next_state, reward)` where:
    /// - `next_state` has shape `[state_dim]` (or `[batch, state_dim]`)
    /// - `reward` has shape `[]` (or `[batch]`) in `(-1, 1)` due to `tanh`
    pub fn forward(&self, state: &Tensor, action: &Tensor) -> (Tensor, Tensor) {
        let state_unbatched = state.size().len() == 1;
        let action_unbatched = action.size().len() == 1;

        assert_eq!(
            state_unbatched, action_unbatched,
            "state and action must both be unbatched or both be batched"
        );

        let state_x = if state_unbatched {
            state.unsqueeze(0)
        } else {
            state.shallow_clone()
        };

        let action_x = if action_unbatched {
            action.unsqueeze(0)
        } else {
            action.shallow_clone()
        };

        let action_dim = action_x.size().last().copied().unwrap_or_default();
        assert_eq!(
            action_dim, NUM_MOVES,
            "action vector must have size {NUM_MOVES}, got {action_dim}"
        );

        assert_eq!(
            state_x.size()[0],
            action_x.size()[0],
            "state and action batch dimensions must match"
        );

        let x = Tensor::cat(&[state_x, action_x], -1);
        let hidden = self.trunk.forward(&x);

        let next_state = self.next_state_head.forward(&hidden);
        let reward = self.reward_head.forward(&hidden).squeeze_dim(-1).tanh();

        if state_unbatched {
            (next_state.squeeze_dim(0), reward.squeeze_dim(0))
        } else {
            (next_state, reward)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::{NUM_MOVES, START_FEN},
        engine::implementations::repr::RepresentationNet,
        game::chessboard::Chessboard,
    };
    use tch::{nn, Kind, Tensor};

    #[test]
    fn test_dynamics_net_output_shapes() {
        let state_dim: i64 = 64;
        let hidden_dim: i64 = 32;

        let vs = nn::VarStore::new(tch::Device::Cpu);
        let net = DynamicsNet::new(vs.root() / "dyn", state_dim, hidden_dim);

        // Unbatched
        let state = Tensor::randn([state_dim], (Kind::Float, tch::Device::Cpu));
        let action = Tensor::zeros([NUM_MOVES], (Kind::Float, tch::Device::Cpu));
        let _ = action.narrow(0, 0, 1).fill_(1.0);

        let (next_state, reward) = net.forward(&state, &action);
        assert_eq!(next_state.size(), vec![state_dim]);
        assert_eq!(reward.size(), Vec::<i64>::new()); // scalar

        // Batched
        let state_b = Tensor::randn([4, state_dim], (Kind::Float, tch::Device::Cpu));
        let action_b = Tensor::randn([4, NUM_MOVES], (Kind::Float, tch::Device::Cpu));

        let (next_state_b, reward_b) = net.forward(&state_b, &action_b);
        assert_eq!(next_state_b.size(), vec![4, state_dim]);
        assert_eq!(reward_b.size(), vec![4]);
    }

    #[test]
    fn test_repr_then_dynamics_end_to_end() {
        let state_dim: i64 = 64;
        let hidden_dim: i64 = 32;

        let board = Chessboard::from_fen(START_FEN, " ");
        let input = board.to_tensor();

        let vs = nn::VarStore::new(input.device());

        // Representation network
        let repr_net = RepresentationNet::new(vs.root() / "repr", 32, state_dim, 3);
        let state = repr_net.forward(&input); // shape: [state_dim]
        assert_eq!(state.size(), vec![state_dim]);

        // Dynamics network (state_dim == output_dim of repr)
        let dyn_net = DynamicsNet::new(vs.root() / "dyn", state_dim, hidden_dim);

        // One-hot action vector in [64 * 64]
        let from: i64 = 12;
        let to: i64 = 28;
        let action_index = from * 64 + to;

        let action = Tensor::zeros([NUM_MOVES], (Kind::Float, input.device()));
        let _ = action.narrow(0, action_index, 1).fill_(1.0);

        let (next_state, reward) = dyn_net.forward(&state, &action);

        assert_eq!(next_state.size(), vec![state_dim]);
        assert_eq!(
            reward.size(),
            Vec::<i64>::new(),
            "reward should be a scalar"
        );

        let r: f64 = reward.double_value(&[]);
        assert!(r > -1.0 && r < 1.0, "reward should be in (-1, 1), got {r}");
    }
}
