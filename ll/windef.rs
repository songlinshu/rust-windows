use ll::platform::*;
/* automatically generated by rust-bindgen */

pub type DWORD = c_ulong;
pub type WINBOOL = c_int;
pub type PWINBOOL = *mut c_int;
pub type LPWINBOOL = *mut c_int;
pub type BOOL = WINBOOL;
pub type BYTE = c_uchar;
pub type PBOOL = *mut BOOL;
pub type LPBOOL = *mut BOOL;
pub type WORD = c_ushort;
pub type FLOAT = c_float;
pub type PFLOAT = *mut FLOAT;
pub type PBYTE = *mut BYTE;
pub type LPBYTE = *mut BYTE;
pub type PINT = *mut c_int;
pub type LPINT = *mut c_int;
pub type PWORD = *mut WORD;
pub type LPWORD = *mut WORD;
pub type LPLONG = *mut c_long;
pub type PDWORD = *mut DWORD;
pub type LPDWORD = *mut DWORD;
pub type PCVOID = *c_void;
pub type LPCVOID = *c_void;
pub type INT = c_int;
pub type UINT = c_uint;
pub type PUINT = *mut c_uint;
pub type LPUINT = *mut c_uint;
pub type WPARAM = UINT_PTR;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;
pub type HRESULT = LONG;
pub type ATOM = WORD;
pub type HHOOK = HANDLE;
pub type HGLOBAL = HANDLE;
pub type HLOCAL = HANDLE;
pub type GLOBALHANDLE = HANDLE;
pub type LOCALHANDLE = HANDLE;
pub type HGDIOBJ = *mut c_void;
pub type HACCEL = HANDLE;
pub type HBITMAP = HANDLE;
pub type HBRUSH = HANDLE;
pub type HCOLORSPACE = HANDLE;
pub type HDC = HANDLE;
pub type HGLRC = HANDLE;
pub type HDESK = HANDLE;
pub type HENHMETAFILE = HANDLE;
pub type HFONT = HANDLE;
pub type HICON = HANDLE;
pub type HKEY = HANDLE;
pub type HMONITOR = HANDLE;
pub type HTERMINAL = HANDLE;
pub type HWINEVENTHOOK = HANDLE;
pub type PHKEY = *mut HKEY;
pub type HMENU = HANDLE;
pub type HMETAFILE = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HMODULE = HINSTANCE;
pub type HPALETTE = HANDLE;
pub type HPEN = HANDLE;
pub type HRGN = HANDLE;
pub type HRSRC = HANDLE;
pub type HSTR = HANDLE;
pub type HTASK = HANDLE;
pub type HWND = HANDLE;
pub type HWINSTA = HANDLE;
pub type HKL = HANDLE;
pub type HFILE = c_int;
pub type HCURSOR = HICON;
pub type COLORREF = DWORD;
pub type FARPROC = *u8;
pub type NEARPROC = *u8;
pub type PROC = *u8;
pub struct Struct_tagRECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}
pub type RECT = Struct_tagRECT;
pub type PRECT = *mut Struct_tagRECT;
pub type LPRECT = *mut Struct_tagRECT;
pub type LPCRECT = *RECT;
pub struct Struct_tagRECTL {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}
pub type RECTL = Struct_tagRECTL;
pub type PRECTL = *mut Struct_tagRECTL;
pub type LPRECTL = *mut Struct_tagRECTL;
pub type LPCRECTL = *RECTL;
pub struct Struct_tagPOINT {
    pub x: LONG,
    pub y: LONG,
}
pub type POINT = Struct_tagPOINT;
pub type POINTL = Struct_tagPOINT;
pub type PPOINT = *mut Struct_tagPOINT;
pub type LPPOINT = *mut Struct_tagPOINT;
pub type PPOINTL = *mut Struct_tagPOINT;
pub type LPPOINTL = *mut Struct_tagPOINT;
pub struct Struct_tagSIZE {
    pub cx: LONG,
    pub cy: LONG,
}
pub type SIZE = Struct_tagSIZE;
pub type SIZEL = Struct_tagSIZE;
pub type PSIZE = *mut Struct_tagSIZE;
pub type LPSIZE = *mut Struct_tagSIZE;
pub type PSIZEL = *mut Struct_tagSIZE;
pub type LPSIZEL = *mut Struct_tagSIZE;
pub struct Struct_tagPOINTS {
    pub x: SHORT,
    pub y: SHORT,
}
pub type POINTS = Struct_tagPOINTS;
pub type PPOINTS = *mut Struct_tagPOINTS;
pub type LPPOINTS = *mut Struct_tagPOINTS;
