pub(super) fn global() -> Box<dyn super::Global> {
    Box::new(super::FakeGlobal)
}
