{
  description = "A dev shell nix flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs, ... } @inputs :
    let 
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = 
        pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
	    nodejs
	    cargo
	    rustc
	  ];
	};
      

    };
}
