use ::parking_lot::Once;
use ::std::cell::UnsafeCell;
use ::std::mem::MaybeUninit;
use ::std::ops::{ Deref, DerefMut };

pub struct LazyWrap<T, F = &'static dyn Fn() -> T> {
	value: UnsafeCell<MaybeUninit<T>>,
	init: F,
	once: Once
}

impl<T> LazyWrap<T> {
	#[inline]
	pub const fn const_new(init: &'static dyn Fn() -> T) -> Self {
		let value = UnsafeCell::new(MaybeUninit::uninit());
		let once = Once::new();
		Self { value, init, once }
	}
}

impl<T, F> LazyWrap<T, F>
where
	F: Fn() -> T
{
	#[inline]
	pub fn new(init: F) -> Self {
		let value = UnsafeCell::new(MaybeUninit::uninit());
		let once = Once::new();
		Self { value, init, once }
	}

	pub fn ensure_initialised(&self) {
		self.once.call_once(|| {
			let value = (self.init)();
			unsafe { (*self.value.get()).write(value) };
		});
	}

	#[inline]
	pub fn is_initialised(&self) -> bool {
		self.once.state().done()
	}
}

impl<T, F> Deref for LazyWrap<T, F>
where
	F: Fn() -> T
{
	type Target = T;
	#[inline]
	fn deref(&self) -> &Self::Target {
		self.ensure_initialised();
		unsafe { (*self.value.get()).assume_init_ref() }
	}
}

impl<T, F> DerefMut for LazyWrap<T, F>
where
	F: Fn() -> T
{
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.ensure_initialised();
		unsafe { (*self.value.get()).assume_init_mut() }
	}
}

unsafe impl<T, F> Send for LazyWrap<T, F> where T: Send {}
unsafe impl<T, F> Sync for LazyWrap<T, F> where T: Sync {}

impl<T, F> Drop for LazyWrap<T, F> {
	#[inline]
	fn drop(&mut self) {
		if self.once.state().done() {
			unsafe { self.value.get_mut().assume_init_drop() }
		}
	}
}
