using System.Runtime.InteropServices;

namespace MovePointerToCenter.WindowsApp;

public static class WindowsApi
{
	[DllImport("user32.dll", EntryPoint = "SetCursorPos", SetLastError = true)]
	[return: MarshalAs(UnmanagedType.Bool)]
	public static extern bool SetCursorPosition(int x, int y);

	public static int GetPrimaryScreenWidth()
		=> GetSystemMetrics(SystemMetrics.PrimaryScreenWidth);

	public static int GetPrimaryScreenHeight()
		=> GetSystemMetrics(SystemMetrics.PrimaryScreenHeight);

	[DllImport("user32.dll", EntryPoint = "GetSystemMetrics", SetLastError = true)]
	private static extern int GetSystemMetrics(int nIndex);

	private static class SystemMetrics
	{
		/// <summary>
		/// SM_CXSCREEN
		/// </summary>
		public const int PrimaryScreenWidth = 0;

		/// <summary>
		/// SM_CYSCREEN
		/// </summary>
		public const int PrimaryScreenHeight = 1;
	}
}
