```fish
# Offloading app files to external hard drive.

# I. Initial setup.

set USER_NAME   $USER
set DISK_NAME   <mounted disk label>

set HDD_PATH    /media/$USER_NAME/$DISK_NAME/Linyx

set FLT_PATH    $HDD_PATH/Flatpak
set FLT_CACHE   $FLT_PATH/user_cache
set FLT_SHARE   $FLT_PATH/user_share
set FLT_APPS    $FLT_PATH/user_var

set UMU_PATH    $HDD_PATH/umu
set UMU_SHARE   $UMU_PATH/user_share

# 1. Create directories on HDD.
mkdir -p $FLT_CACHE
mkdir -p $FLT_SHARE
mkdir -p $FLT_APPS
mkdir -p $UMU_SHARE

# 2. Setup symlinks.
function setup_symlink --argument-names from to
    rm -rf $to
    ln -vs $from $to
end

setup_symlink $FLT_CACHE ~/.local/cache/flatpak
setup_symlink $FLT_SHARE ~/.local/share/flatpak
setup_symlink $UMU_SHARE ~/.local/share/umu

# 3. Add flathub for user.
flatpak remote-add --user flathub https://dl.flathub.org/repo/flathub.flatpakrepo

# 4. Always use --user when installing.
set APP_ID net.lutris.Lutris

setup_symlink $FLT_APPS/$APP_ID
flatpak install --user flathub $APP_ID
```
