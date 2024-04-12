use std::borrow::Cow;
#[cfg(unix)]
use std::path::Path;
#[cfg(windows)]
use widestring::U16CStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NameInner<'s> {
	#[cfg(windows)]
	NamedPipe(Cow<'s, U16CStr>),
	#[cfg(unix)]
	UdSocketPath(Cow<'s, Path>),
	#[cfg(any(target_os = "linux", target_os = "android"))]
	UdSocketNs(Cow<'s, [u8]>),
}

impl Default for NameInner<'_> {
	fn default() -> Self {
		#[cfg(windows)]
		{
			Self::NamedPipe(Cow::default())
		}
		#[cfg(unix)]
		{
			Self::UdSocketPath(Cow::default())
		}
	}
}

macro_rules! map_cow {
	($nm:ident in $var:expr => $e:expr) => {
		match $var {
			#[cfg(windows)]
			NameInner::NamedPipe($nm) => NameInner::NamedPipe($e),
			#[cfg(unix)]
			NameInner::UdSocketPath($nm) => NameInner::UdSocketPath($e),
			#[cfg(any(target_os = "linux", target_os = "android"))]
			NameInner::UdSocketNs($nm) => NameInner::UdSocketNs($e),
		}
	};
}

impl<'s> NameInner<'s> {
	pub const fn is_namespaced(&self) -> bool {
		match self {
			#[cfg(windows)]
			Self::NamedPipe(..) => true,
			#[cfg(unix)]
			Self::UdSocketPath(..) => todo!(),
			#[cfg(any(target_os = "linux", target_os = "android"))]
			Self::UdSocketNs(..) => true,
		}
	}
	pub const fn is_path(&self) -> bool {
		match self {
			#[cfg(windows)]
			Self::NamedPipe(..) => true,
			#[cfg(unix)]
			Self::UdSocketPath(..) => true,
			#[cfg(any(target_os = "linux", target_os = "android"))]
			Self::UdSocketNs(..) => false,
		}
	}

	#[inline]
	pub fn borrow(&self) -> NameInner<'_> {
		map_cow!(cow in self => Cow::Borrowed(cow))
	}

	pub fn into_owned(self) -> NameInner<'static> {
		map_cow!(cow in self => Cow::Owned(cow.into_owned()))
	}
}