fn main() -> std::io::Result<()> {
    // TODO: Remove types that will be included automatically
    let tokens = windows_macros::generate! {
        Windows::Win32::Foundation::{INVALID_HANDLE_VALUE, CloseHandle},
        Windows::Win32::Storage::FileSystem::{WriteFile, ReadFile, CreateIoCompletionPort},
        Windows::Win32::System::Diagnostics::Debug::WIN32_ERROR,
        Windows::Win32::System::SystemServices::GetOverlappedResult,
        Windows::Win32::System::Threading::CreateEventW,
        Windows::Win32::Storage::FileSystem::{ GetQueuedCompletionStatus, OVERLAPPED_ENTRY, GetQueuedCompletionStatusEx,  PostQueuedCompletionStatus, FlushFileBuffers, FILE_FLAGS_AND_ATTRIBUTES},
        Windows::Win32::Networking::WinSock::{SOCKADDR, SOCKADDR_STORAGE, WSAGetLastError,  SOCKET_ERROR,  SOCKADDR_IN, SOCKADDR_IN6, WSABUF, WSASend, WSAGetOverlappedResult, WSARecv, setsockopt, SOL_SOCKET,  WSARecvFrom, WSASendTo, WSAIoctl},
        Windows::Win32::NetworkManagement::IpHelper::ADDRESS_FAMILY,
        Windows::Win32::System::Pipes::{ WaitNamedPipeW, CreatePipe, ConnectNamedPipe, DisconnectNamedPipe, NAMED_PIPE_INFO_FLAGS, CreateNamedPipeW},
        Windows::Win32::System::WindowsProgramming::{PIPE_ACCESS_DUPLEX, PIPE_UNLIMITED_INSTANCES, PIPE_ACCESS_INBOUND,  PIPE_ACCESS_OUTBOUND,  PIPE_REJECT_REMOTE_CLIENTS, INFINITE},
    };

    let mut path: std::path::PathBuf = windows_reader::workspace_dir().into();
    path.push("src");
    path.push("bindings.rs");

    std::fs::write(&path, tokens)?;

    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(&path);
    cmd.output()?;

    Ok(())
}
