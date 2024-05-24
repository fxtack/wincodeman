# wincodeman
Through this tool, you can quickly query the conversion values and error messages between NTSTATUS, Win32 Error and HRESULT.

Windows result/error code relationship:

<img src="https://github.com/fxtack/wincodeman/assets/59989422/d4d0e89a-7b75-4e44-ad4c-2ae9403fe6a3" width="40%"/>

## Usage

```
wcm.exe --help
wcm v0.1.0.078500ff
tool for query the windows error code information

USAGE:
    wcm.exe <--ntstatus <NTSTATUS>|--win32err <Win32Error>|--hresult <HRESULT>>

OPTIONS:
    -N, --ntstatus <NTSTATUS>      NTSTATUS code
    -E, --win32err <Win32Error>    Win32 error code
    -H, --hresult <HRESULT>        HRESULT error code
    -h, --help                     Print help information
    -v, --version                  Prints version information
```

## Example

### Query NTSTATUS '0xc0000022'

```
PS C:\wincodeman> wcm.exe --ntstatus 0xc0000022
   NTSTATUS: 0xc0000022
Win32 Error: 5
    HRESULT: 0x80070005
    Message: {Access Denied}
A process has requested access to an object, but has not been granted those access rights.
```
