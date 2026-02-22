use tch::{nn, nn::Module, Tensor};

use super::blocks::mlp_block;

const BOARD_CHANNELS: i64 = 14;
const BOARD_SIZE: i64 = 8;

/// Representation network for MuZero.
///
/// Takes a raw chessboard tensor (14-channel 8×8 bitboard planes) and produces
/// a compact hidden-state embedding vector.
///
/// Supports both unbatched (`[14, 8, 8]`) and batched (`[batch, 14, 8, 8]`) inputs.
pub struct RepresentationNet {
    net: Box<dyn Module>,
}

impl RepresentationNet {
    pub fn new(path: nn::Path, hidden_dim: i64, output_dim: i64, kernel_size: i64) -> Self {
        assert!(kernel_size > 0, "kernel_size must be > 0");
        assert!(
            kernel_size % 2 == 1,
            "kernel_size must be odd to preserve 8x8 board shape"
        );

        let conv_cfg = nn::ConvConfig {
            padding: kernel_size / 2,
            ..Default::default()
        };

        let net = Box::new(
            nn::seq()
                .add(nn::conv2d(
                    path.clone() / "conv1",
                    BOARD_CHANNELS,
                    hidden_dim,
                    kernel_size,
                    conv_cfg,
                ))
                .add_fn(|xs| xs.gelu("none"))
                .add(nn::conv2d(
                    path.clone() / "conv2",
                    hidden_dim,
                    hidden_dim,
                    kernel_size,
                    conv_cfg,
                ))
                .add_fn(|xs| xs.gelu("none"))
                .add_fn(|xs| xs.flatten(1, -1))
                .add(mlp_block(
                    path.clone() / "proj",
                    hidden_dim * BOARD_SIZE * BOARD_SIZE,
                    hidden_dim,
                ))
                .add(nn::linear(
                    path / "head",
                    hidden_dim,
                    output_dim,
                    Default::default(),
                )),
        );

        Self { net }
    }

    /// Forward pass.
    ///
    /// `input` can be:
    /// - unbatched: `[14, 8, 8]` → returns `[output_dim]`
    /// - batched: `[batch, 14, 8, 8]` → returns `[batch, output_dim]`
    pub fn forward(&self, input: &Tensor) -> Tensor {
        let size = input.size();

        match size.as_slice() {
            [channels, height, width] => {
                assert_eq!(
                    *channels, BOARD_CHANNELS,
                    "expected {} channels, got {}",
                    BOARD_CHANNELS, channels
                );
                assert_eq!(
                    *height, BOARD_SIZE,
                    "expected board height {}, got {}",
                    BOARD_SIZE, height
                );
                assert_eq!(
                    *width, BOARD_SIZE,
                    "expected board width {}, got {}",
                    BOARD_SIZE, width
                );

                self.net.forward(&input.unsqueeze(0)).squeeze_dim(0)
            }
            [_, channels, height, width] => {
                assert_eq!(
                    *channels, BOARD_CHANNELS,
                    "expected {} channels, got {}",
                    BOARD_CHANNELS, channels
                );
                assert_eq!(
                    *height, BOARD_SIZE,
                    "expected board height {}, got {}",
                    BOARD_SIZE, height
                );
                assert_eq!(
                    *width, BOARD_SIZE,
                    "expected board width {}, got {}",
                    BOARD_SIZE, width
                );

                self.net.forward(input)
            }
            _ => panic!(
                "expected input shape [14, 8, 8] or [batch, 14, 8, 8], got {:?}",
                size
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RepresentationNet;
    use crate::{constants::START_FEN, game::chessboard::Chessboard};
    use tch::nn;

    #[test]
    fn test_embed_chessboard_forward_from_chessboard_tensor() {
        let board = Chessboard::from_fen(START_FEN, " ");
        let input = board.to_tensor();

        let vs = nn::VarStore::new(input.device());
        let model = RepresentationNet::new(vs.root(), 32, 64, 3);
        let output = model.forward(&input);

        assert_eq!(output.size(), vec![64]);
    }

    #[test]
    fn test_embed_chessboard_forward_from_batched_tensor() {
        let board = Chessboard::from_fen(START_FEN, " ");
        let input = board.to_tensor().unsqueeze(0);

        let vs = nn::VarStore::new(input.device());
        let model = RepresentationNet::new(vs.root(), 32, 64, 3);
        let output = model.forward(&input);

        assert_eq!(output.size(), vec![1, 64]);
    }

    #[test]
    #[should_panic(expected = "expected input shape [14, 8, 8] or [batch, 14, 8, 8]")]
    fn test_embed_chessboard_forward_rejects_invalid_rank() {
        let input = tch::Tensor::zeros([14, 8], (tch::Kind::Float, tch::Device::Cpu));

        let vs = nn::VarStore::new(input.device());
        let model = RepresentationNet::new(vs.root(), 32, 64, 3);

        let _ = model.forward(&input);
    }
}
