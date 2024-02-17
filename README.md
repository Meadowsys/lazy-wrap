# lazy-wrap

Smol wrapper around a type with an attached initialisation function. On first access, it'll call the attached function to initialise the value. Implements `Deref` and `DerefMut`.

You can create this in a `static` using `const_new`. I've not really found any better solution than to take a static reference, so you would need to borrow on your closure. This would look something like:

```rs
static STATIC_STRING: LazyWrap<String> = LazyWrap::const_new(&|| {
   // do whatever
   "hallo!!!".into()
});
```

Please file issues if you find any, they would be very much appreciated c:
