@echo off

SET exec=cargo xbuild
SET "release="

IF "%1" == "release" (
  SET release=--release
  IF "%2" == "boot" SET exec=bootimage build
) ELSE (
  IF "%1" == "boot" (
    SET exec=bootimage build
    IF "%2" == "release" SET release=--release
  )
)

@echo on
%exec% %release% --target x86_64-plox_os.json
@echo off

SET "exec="
SET "release="
