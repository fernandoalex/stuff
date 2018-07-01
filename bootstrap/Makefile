include Makefile.tools

BIN_FOLDER = ~/bin

setup:
	echo 'Installing Brew'
	/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
	brew install wget
	brew install python3
	mkdir $(BIN_FOLDER)

tools:
	wget -P $(BIN_FOLDER)/ https://releases.hashicorp.com/packer/$(packer)/packer_$(packer)_darwin_amd64.zip
	unzip $(BIN_FOLDER)/packer_$(packer)_darwin_amd64.zip -d $(BIN_FOLDER)
	wget -P $(BIN_FOLDER)/ https://releases.hashicorp.com/terraform/$(terraform)/terraform_$(terraform)_darwin_amd64.zip
	unzip $(BIN_FOLDER)/terraform_$(terraform)_darwin_amd64.zip -d $(BIN_FOLDER)