mod data_type;
mod memory;

#[macro_use]
extern crate log;

use common::utok;

pub use data_type::DataType;

pub trait Llama2 {
    fn hidden_size(&self) -> usize;
    fn intermediate_size(&self) -> usize;
    fn max_position_embeddings(&self) -> usize;
    fn num_attention_heads(&self) -> usize;
    fn num_hidden_layers(&self) -> usize;
    fn num_key_value_heads(&self) -> usize;
    fn vocab_size(&self) -> usize;
    fn data_type(&self) -> DataType;

    fn embed_tokens(&self) -> &[u8];
    fn input_layernorm(&self, layer: usize) -> &[u8];
    fn self_attn_q_proj(&self, layer: usize) -> &[u8];
    fn self_attn_k_proj(&self, layer: usize) -> &[u8];
    fn self_attn_v_proj(&self, layer: usize) -> &[u8];
    fn self_attn_o_proj(&self, layer: usize) -> &[u8];
    fn post_attention_layernorm(&self, layer: usize) -> &[u8];
    fn mlp_gate(&self, layer: usize) -> &[u8];
    fn mlp_down(&self, layer: usize) -> &[u8];
    fn mlp_up(&self, layer: usize) -> &[u8];
    fn model_norm(&self) -> &[u8];
    fn lm_head(&self) -> &[u8];
}

pub use memory::{Memory, SafeTensorError};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ConfigJson {
    pub bos_token_id: utok,
    pub eos_token_id: utok,
    pub hidden_size: usize,
    pub intermediate_size: usize,
    pub max_position_embeddings: usize,
    pub num_attention_heads: usize,
    pub num_hidden_layers: usize,
    pub num_key_value_heads: usize,
    pub vocab_size: usize,
    pub rms_norm_eps: f32,
    pub rope_theta: f32,
    pub torch_dtype: DataType,
}

struct LayerParamsOffset {
    input_layernorm: usize,
    self_attn_q_proj: usize,
    self_attn_k_proj: usize,
    self_attn_v_proj: usize,
    self_attn_o_proj: usize,
    post_attention_layernorm: usize,
    mlp_gate: usize,
    mlp_down: usize,
    mlp_up: usize,
}
