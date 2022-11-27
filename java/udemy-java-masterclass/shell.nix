{ pkgs ? import <unstable> {} }:
pkgs.mkShell {
	nativeBuildInputs = [
		pkgs.jdk17
		pkgs.java-language-server 
	];
}
