pub trait FixedSize {
    type Scalar;
}

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix<T, const R: usize, const C: usize> where [(); R * C]:
{
    pub data: [T; R * C]
}

impl<T: Default + Copy, const R: usize, const C: usize> Default for Matrix<T, R, C> where [(); R * C]: {
    fn default() -> Self {
	Self {
	    data: [T::default(); R * C],
	}
    }
}

impl<T, const R: usize, const C: usize> FixedSize for Matrix<T, R, C> where [(); R * C]: {
    type Scalar = T;
}

#[repr(C)]
#[derive(Debug)]
pub struct Map<'a, T: FixedSize> {
    pub data: *const T::Scalar,
    _pd: std::marker::PhantomData<&'a ()>,
}

#[repr(C)]
#[derive(Debug)]
pub struct MapMut<'a, T: FixedSize> {
    pub data: *mut T::Scalar,
    _pd: std::marker::PhantomData<&'a ()>,
}

#[macro_export]
macro_rules! impl_to_string {
    ($name:ident$(<$lt:lifetime>)?, $link_name:ident) => {
	paste::paste! {
	    impl $name$(<$lt>)? {
		pub fn to_cxx_string(&self) -> rxx::UniqueString {
		    extern "C" {
			#[link_name=stringify!([< eigen_rxx_ $link_name _to_string>])]
			fn __func(this: &$name$(<$lt>)?, ret: *mut rxx::UniqueString);
		    }

		    unsafe {
		        let mut __ret = std::mem::MaybeUninit::<rxx::UniqueString>::uninit();
			let mut __ret_ptr = __ret.as_mut_ptr();
			__func(self, __ret_ptr);
			__ret.assume_init()
		    }
		}
	    }

	    impl std::fmt::Display for $name$(<$lt>)? {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		    let s = self.to_cxx_string();
		    write!(f, "{}", s.to_str())
		}
	    }
	}
    };

    ($name:ident$(<$lt:lifetime>)?) => {
	impl_to_string!($name$(<$lt>)?, $name);
    }
}

#[macro_export]
macro_rules! impl_fixed_mat {
    ($name:ident = Matrix<$tp:ty, $rows:literal, $cols:literal>) => {
	paste::paste! {
	    pub type $name = $crate::Matrix<$tp, $rows, $cols>;
	    type [<$name _from>] = nalgebra::Matrix<$tp, nalgebra::dimension::Const<$rows>, nalgebra::dimension::Const<$cols>, nalgebra::ArrayStorage<$tp, $rows, $cols>>;

	    impl_to_string!($name);

	    impl From<[<$name _from>]> for $name {
		fn from(v: [<$name _from>]) -> Self {
		    Self {
			data: v.as_slice().try_into().unwrap()
		    }
		}
	    }

	    impl From<$name> for [<$name _from>] {
		fn from(v: $name) -> Self {
		    let d = unsafe {
			std::mem::transmute::<[$tp; $rows * $cols], [[$tp; $rows]; $cols]>(v.data)
		    };
		    Self::from_data(nalgebra::ArrayStorage(d))
		}
	    }


	    // MapMut
	    pub type [<MapMut_ $name>]<'a> = $crate::MapMut<'a, $name>;
	    type [<MapMut_ $name _from>]<'a> = nalgebra::MatrixSliceMut<'a, $tp, nalgebra::dimension::Const<$rows>, nalgebra::dimension::Const<$cols>>;

	    impl_to_string!([<MapMut_ $name>]<'_>);

	    impl<'a> [<MapMut_ $name>]<'a> {
		pub fn new(data: &'a mut [$tp]) -> Self {
		    extern "C" {
			#[link_name=stringify!([< eigen_rxx_MapMut_ $name _new>])]
			fn __func<'a>(data: *mut $tp, ret: *mut [<MapMut_ $name>]<'a>);
		    }

		    unsafe {
		        let mut __ret = std::mem::MaybeUninit::<Self>::uninit();
			__func(data.as_mut_ptr(), __ret.as_mut_ptr());
			__ret.assume_init()
		    }
		}
	    }

	    impl<'a> From<[<MapMut_ $name _from>]<'a>> for [<MapMut_ $name>]<'a> {
		fn from(v: [<MapMut_ $name _from>]<'a>) -> Self {
		    Self::new(v.data.into_slice_mut())
		}
	    }

	    impl<'a> From<[<MapMut_ $name>]<'a>> for [<MapMut_ $name _from>]<'a> {
		fn from(v: [<MapMut_ $name>]<'a>) -> Self {
		    // stride BUG strides: (CStride, RStride)
		    let s = unsafe {
			nalgebra::SliceStorageMut::from_raw_parts(v.data, (nalgebra::Dim::from_usize($rows), nalgebra::Dim::from_usize($cols)), (nalgebra::Dim::from_usize(1), nalgebra::Dim::from_usize($cols)))
		    };
		    Self::from_data(s)
		}
	    }

	    // Map
	    pub type [<Map_ $name>]<'a> = $crate::Map<'a, $name>;
	    type [<Map_ $name _from>]<'a> = nalgebra::MatrixSlice<'a, $tp, nalgebra::dimension::Const<$rows>, nalgebra::dimension::Const<$cols>>;

	    impl_to_string!([<Map_ $name>]<'_>);

	    impl<'a> Map<'a, $name> {
		pub fn new(data: &'a [$tp]) -> Self {
		    extern "C" {
			#[link_name=stringify!([< eigen_rxx_Map_ $name _new>])]
			fn __func<'a>(data: *const $tp, ret: *mut Map<'a, $name>);
		    }

		    unsafe {
		        let mut __ret = std::mem::MaybeUninit::<Self>::uninit();
			__func(data.as_ptr(), __ret.as_mut_ptr());
			__ret.assume_init()
		    }
		}
	    }

	    impl<'a> From<[<Map_ $name _from>]<'a>> for Map<'a, $name> {
		fn from(v: [<Map_ $name _from>]<'a>) -> Self {
		    Self::new(v.data.into_slice())
		}
	    }

	    impl<'a> From<Map<'a, $name>> for [<Map_ $name _from>]<'a> {
		fn from(v: Map<'a, $name>) -> Self {
		    // stride BUG strides: (CStride, RStride)
		    let s = unsafe {
			nalgebra::SliceStorage::from_raw_parts(v.data, (nalgebra::Dim::from_usize($rows), nalgebra::Dim::from_usize($cols)), (nalgebra::Dim::from_usize(1), nalgebra::Dim::from_usize($cols)))
		    };
		    Self::from_data(s)
		}
	    }

	}
    }
}

impl_fixed_mat!(Matrix2d = Matrix<f64, 2, 2>);
impl_fixed_mat!(Matrix3d = Matrix<f64, 3, 3>);
impl_fixed_mat!(Matrix4d = Matrix<f64, 4, 4>);

impl_fixed_mat!(Matrix2i = Matrix<i32, 2, 2>);

impl_fixed_mat!(Vector2d = Matrix<f64, 2, 1>);
impl_fixed_mat!(Vector3d = Matrix<f64, 3, 1>);
impl_fixed_mat!(Vector4d = Matrix<f64, 4, 1>);

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Quaternion<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> FixedSize for Quaternion<T> {
    type Scalar = T;
}

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct AngleAxis<T> {
    pub axis: Vector3d,
    pub angle: T
}

#[macro_export]
macro_rules! impl_quat {
    ($name:ident = Quaternion<$tp:ty>) => {
	paste::paste! {
	    pub type $name = $crate::Quaternion<$tp>;
	    type [<$name _from>] = nalgebra::Quaternion<$tp>;

	    impl From<[<$name _from>]> for $name {
		fn from(v: [<$name _from>]) -> Self {
		    Self {
			x: v.coords.x,
			y: v.coords.y,
			z: v.coords.z,
			w: v.coords.w,
		    }
		}
	    }

	    impl std::ops::Mul for $name {
		type Output = Self;

		fn mul(self, other: Self) -> Self::Output {
		    extern "C" {
			#[link_name=stringify!([< eigen_rxx_ $name _mul>])]
			fn __func(this: &$name, other: &$name, ret: *mut $name);
		    }

		    unsafe {
			let mut __ret = std::mem::MaybeUninit::<Self::Output>::uninit();
			let mut __ret_ptr = __ret.as_mut_ptr();
			__func(&self, &other, __ret_ptr);
			__ret.assume_init()
		    }
		}
	    }

	    impl From<$name> for [<$name _from>] {
		fn from(v: $name) -> Self {
		    Self::new(v.w, v.x, v.y, v.z)
		}
	    }

	    pub type [<MapMut_ $name>]<'a> = $crate::MapMut<'a, $name>;
	    type [<MapMut_ $name _from>] = nalgebra::Quaternion<$tp>;

	    impl<'a> [<MapMut_ $name>]<'a> {
		pub fn new(data: &'a mut [$tp]) -> Self {
		    extern "C" {
			#[link_name=stringify!([< eigen_rxx_MapMut_ $name _new>])]
			fn __func<'a>(data: *mut $tp, ret: *mut [<MapMut_ $name>]<'a>);
		    }

		    unsafe {
		        let mut __ret = std::mem::MaybeUninit::<Self>::uninit();
			__func(data.as_mut_ptr(), __ret.as_mut_ptr());
			__ret.assume_init()
		    }

		}
	    }

	    impl<'a> From<&'a mut [<MapMut_ $name _from>]> for [<MapMut_ $name>]<'a> {
		fn from(v: &'a mut [<MapMut_ $name _from>]) -> Self {
		    Self::new(v.coords.as_mut_slice())
		}
	    }

	    impl<'a> From<[<MapMut_ $name>]<'a>> for [<MapMut_ $name _from>] {
		fn from(v: [<MapMut_ $name>]<'a>) -> Self {
		    let d = unsafe {
			std::slice::from_raw_parts(v.data, 4)
		    };
		    Self::from_vector(nalgebra::Vector4::from_column_slice(d))
		}
	    }

	    pub type [<Map_ $name>]<'a> = $crate::Map<'a, $name>;
	    type [<Map_ $name _from>] = nalgebra::Quaternion<$tp>;

	    impl<'a> [<Map_ $name>]<'a> {
		pub fn new(data: &'a  [$tp]) -> Self {
		    extern "C" {
			#[link_name=stringify!([< eigen_rxx_Map_ $name _new>])]
			fn __func<'a>(data: *const $tp, ret: *mut [<Map_ $name>]<'a>);
		    }

		    unsafe {
		        let mut __ret = std::mem::MaybeUninit::<Self>::uninit();
			__func(data.as_ptr(), __ret.as_mut_ptr());
			__ret.assume_init()
		    }

		}
	    }

	    impl<'a> From<&'a  [<Map_ $name _from>]> for [<Map_ $name>]<'a> {
		fn from(v: &'a  [<Map_ $name _from>]) -> Self {
		    Self::new(v.coords.as_slice())
		}
	    }

	    impl<'a> From<[<Map_ $name>]<'a>> for [<Map_ $name _from>] {
		fn from(v: [<Map_ $name>]<'a>) -> Self {
		    let d = unsafe {
			std::slice::from_raw_parts(v.data, 4)
		    };
		    Self::from_vector(nalgebra::Vector4::from_column_slice(d))
		}
	    }
	}
    }
}

impl_quat!(Quaterniond=Quaternion<f64>);
