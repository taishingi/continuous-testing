packer {
  required_plugins {
    docker = {
      version = ">= 0.0.7"
      source  = "github.com/hashicorp/docker"
    }
  }
}

variable "image_stable" {
  type    = string
  default = "instrumentisto/rust:buster"
}

variable "image_beta" {
  type    = string
  default = "instrumentisto/rust:beta"
}

variable "image_nightly" {
  type    = string
  default = "instrumentisto/rust:nightly"
}

source "docker" "stable" {
  image  = var.image_stable
  commit = true
}

source "docker" "beta" {
  image  = var.image_beta
  commit = true
}

source "docker" "nightly" {
  image  = var.image_nightly
  commit = true
}


build {
  name = "stable"
  source "source.docker.stable" {
  }

  provisioner "shell" {
    scripts = ["stable.sh"]
  }
}

build {
  name = "beta"
    source "source.docker.beta" {
  }

  provisioner "shell" {
    scripts = ["beta.sh"]
  }
}

build {
  name = "nightly"
  source "source.docker.nightly" {
  }

  provisioner "shell" {
    scripts = ["nightly.sh"]
  }
}

