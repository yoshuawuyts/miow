fn main() -> std::io::Result<()> {
    let tokens = windows::generate! {
        Windows::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
        Windows::Win32::NetworkManagement::IpHelper::ADDRESS_FAMILY,
        Windows::Win32::Networking::WinSock::{
            setsockopt, WSAGetLastError, WSAGetOverlappedResult, WSAIoctl, WSARecv, WSARecvFrom,
            WSASend, WSASendTo, LPFN_ACCEPTEX, LPFN_CONNECTEX, LPFN_GETACCEPTEXSOCKADDRS,
            SOCKADDR_IN, SOCKADDR_IN6, SOCKADDR_STORAGE, SOCKET_ERROR, SOL_SOCKET,
        },
        Windows::Win32::Storage::FileSystem::{CreateIoCompletionPort, ReadFile, WriteFile},
        Windows::Win32::Storage::FileSystem::{
            FlushFileBuffers, GetQueuedCompletionStatus, GetQueuedCompletionStatusEx,
            PostQueuedCompletionStatus, FILE_FLAGS_AND_ATTRIBUTES,
        },
        Windows::Win32::System::Diagnostics::Debug::WIN32_ERROR,
        Windows::Win32::System::Pipes::{
            ConnectNamedPipe, CreateNamedPipeW, CreatePipe, DisconnectNamedPipe, WaitNamedPipeW,
            NAMED_PIPE_INFO_FLAGS,
        },
        Windows::Win32::System::SystemServices::GetOverlappedResult,
        Windows::Win32::System::Threading::CreateEventW,
        Windows::Win32::System::WindowsProgramming::{
            INFINITE, PIPE_ACCESS_DUPLEX, PIPE_ACCESS_INBOUND, PIPE_ACCESS_OUTBOUND,
            PIPE_REJECT_REMOTE_CLIENTS, PIPE_UNLIMITED_INSTANCES,
        },
    };

    let mut path: std::path::PathBuf = windows::workspace_dir().into();
    path.push("src");
    path.push("bindings.rs");

    std::fs::write(&path, tokens)?;

    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(&path);
    cmd.output()?;

    Ok(())
}
