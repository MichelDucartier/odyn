use tch::nn::{self, Module};

const BOARD_CHANNELS: i64 = 14;
const BOARD_SIZE: i64 = 8;

pub fn mlp_block(path: nn::Path, input_dim: i64, output_dim: i64) -> impl Module {
    nn::seq()
        .add(nn::linear(
            path / "mlp_block",
            input_dim,
            output_dim,
            Default::default(),
        ))
        .add_fn(|xs| xs.gelu("none"))
}

pub fn embed_chessboard(
    path: nn::Path,
    hidden_dim: i64,
    output_dim: i64,
    kernel_size: i64,
) -> impl Module {
    assert!(kernel_size > 0, "kernel_size must be > 0");
    assert!(
        kernel_size % 2 == 1,
        "kernel_size must be odd to preserve 8x8 board shape"
    );

    let conv_cfg = nn::ConvConfig {
        padding: kernel_size / 2,
        ..Default::default()
    };

    let net = nn::seq()
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
        ));

    nn::func(move |xs| {
        if xs.size().len() == 3 {
            net.forward(&xs.unsqueeze(0)).squeeze_dim(0)
        } else {
            net.forward(xs)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::embed_chessboard;
    use crate::{constants::START_FEN, game::chessboard::Chessboard};
    use tch::{nn, nn::Module};

    #[test]
    fn test_embed_chessboard_forward_from_chessboard_tensor() {
        let board = Chessboard::from_fen(START_FEN, " ");
        let input = board.to_tensor();

        let vs = nn::VarStore::new(input.device());
        let model = embed_chessboard(vs.root(), 32, 64, 3);
        let output = model.forward(&input);

        assert_eq!(output.size(), vec![64]);
    }
}
