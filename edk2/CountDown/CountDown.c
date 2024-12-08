#include <Uefi.h>
#include <Library/UefiBootServicesTableLib.h>
#include <Library/UefiApplicationEntryPoint.h>
#include <Library/UefiLib.h>
#include <Library/UefiRuntimeServicesTableLib.h>

EFI_STATUS
EFIAPI
UefiMain (
  IN EFI_HANDLE        ImageHandle,
  IN EFI_SYSTEM_TABLE  *SystemTable
  )
{
  for (int i = 5; i > 0; i--) {
    Print(L"%d \n", i);
    gBS->Stall(1 * 1000 * 1000);
  }

  gRT->ResetSystem (EfiResetShutdown, EFI_SUCCESS, 0, NULL);

  return EFI_SUCCESS;
}
