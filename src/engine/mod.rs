use crate::pb::Spec;
use image::ImageOutputFormat;

mod photon;
pub use photon::Photon;

pub trait Engine {
  // 按照 specs 进行一系列有序处理
  fn apply(&mut self, specs: &[Spec]);
  // 从 engine 中生成目标图片
  fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

// SpecTransform： 未来扩展更多 spec，实现即可
pub trait SpecTransform<T> {
  // 对图片使用 op 做 transform
  fn transform(&mut self, op: T);
}