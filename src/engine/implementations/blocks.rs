use tch::nn::{self, Module};

/// A single linear layer followed by GELU activation.
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
