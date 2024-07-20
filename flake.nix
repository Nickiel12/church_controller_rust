/*
TODO
1. Find and replace "joystick-controller-client" with your package name for **ALL FILES IN REPOSITORY**
2. Add a flake description that describes the workspace on line 27
3. Add a package description on line 70
4. (optional) uncomment `nativeBuildInputs` and `buildInputs` on lines 43 and 44 if you need openssl
5. (optional) set your project homepage, license, and maintainers list on lines 48-51
6. (optional) uncomment the NixOS module and update it for your needs
7. Delete this comment block
*/

/*
Some utility commands:
- `nix flake update --commit-lock-file`
- `nix flake lock update-input <input>`
- `nix build .#joystick-controller-client` or `nix build .`
- `nix run .#joystick-controller-client` or `nix run .`
*/

{
  description = "Joystick Controller client for the VCC motorized camera";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      overlays = [ (import rust-overlay) ];
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      rustSettings = with pkgs; {
        src = ./.;
        nativeBuildInputs = [ pkg-config ];
        buildInputs = [ 
          openssl 
          systemd 
          gtk4
          gst_all_1.gstreamer
          gst_all_1.gst-plugins-base
          gst_all_1.gst-plugins-good
          gst_all_1.gst-plugins-bad
        ];
        cargoHash = nixpkgs.lib.fakeHash;
      };
      meta = with nixpkgs.lib; {
        #homepage = "https://example.com";
        #license = [ licenses.gpl3 ];
        platforms = [ system ];
        #maintainers = with maintainers; [ ];
      };
    in {
      devShells.${system}.default = with pkgs; mkShell {
        packages = [
          (pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" ];
          })
          cargo-edit
          bacon
        ];
        inputsFrom = with self.packages.${system}; [ joystick-controller-client ];
      };
      packages.${system} = {
        default = self.packages.${system}.joystick-controller-client;
        joystick-controller-client = pkgs.rustPlatform.buildRustPackage (rustSettings // {
          pname = "joystick-controller-client";
          version = "0.1.0";
          buildAndTestSubdir = "joystick-controller-client";
          cargoHash = "sha256-+TaGIiKf+Pz2bTABeG8aCZz0/ZTCKl5398+qbas4Nvo=";
          meta = meta // {
            description = "";
          };
        });
      };
      /*
      nixosModules.default = { config, ... }: let
        lib = nixpkgs.lib;
      in {
        options.services.joystick-controller-client = {
          enable = lib.mkEnableOption (lib.mdDoc "joystick-controller-client service");
          package = lib.mkOption {
            type = lib.types.package;
            default = self.packages.${system}.joystick-controller-client;
            defaultText = "pkgs.joystick-controller-client";
            description = lib.mdDoc ''
              The joystick-controller-client package that should be used.
            '';
          };
          port = lib.mkOption {
            type = lib.types.port;
            default = 8000;
            description = lib.mdDoc ''
              The port at which to run.
            '';
          };
        };
        config.systemd.services.joystick-controller-client = let
          cfg = config.services.joystick-controller-client;
          pkg = self.packages.${system}.joystick-controller-client;
        in lib.mkIf cfg.enable {
          description = pkg.meta.description;
          after = [ "network.target" ];
          wantedBy = [ "network.target" ];
          serviceConfig = {
            ExecStart = ''
              ${cfg.package}/bin/joystick-controller-client --port ${builtins.toString cfg.port}
            '';
            Restart = "always";
            DynamicUser = true;
          };
        };
      };
      */
    };
}

