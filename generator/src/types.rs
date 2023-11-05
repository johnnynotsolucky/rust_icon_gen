macro_rules! wrap_string {
	($name:ident) => {
		#[derive(Default, Clone, Debug)]
		pub struct $name(pub String);

		impl ::std::fmt::Display for $name {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", self.0)
			}
		}

		impl From<String> for $name {
			fn from(value: String) -> Self {
				Self(value)
			}
		}

		impl From<&'static str> for $name {
			fn from(value: &'static str) -> Self {
				Self(value.into())
			}
		}
	};
}

wrap_string!(Variant);
wrap_string!(IconName);
wrap_string!(Class);
wrap_string!(ViewBox);
wrap_string!(Stroke);
wrap_string!(Fill);
wrap_string!(Width);
wrap_string!(Height);
wrap_string!(IconsPath);
wrap_string!(StrokeWidth);
wrap_string!(StrokeLinecap);
wrap_string!(StrokeLinejoin);
