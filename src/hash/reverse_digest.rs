pub trait ReversibleDigest  {
  /**
   * Returns a reversed Hash160version of the given digest
   */
  fn reverse(&self) -> Self;
}
