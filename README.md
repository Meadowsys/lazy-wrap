# lazy-wrap

Smol wrapper around a type with an attached initialisation function. On first access, it'll call the attached function to initialise the value. Implements `Deref` and `DerefMut`.

You can create this in a `static` variable. I've not really found any easier solution than to take a `&'static dyn Fn() -> T` as default type for `F` for the initialiser, so you would need to borrow on your closure. This would look something like:

```rs
static STATIC_STRING: LazyWrap<String> = LazyWrap::new(&|| {
   // do whatever
   "hallo!!!".into()
});
```

Please file issues if you find any, they would be very much appreciated c:
