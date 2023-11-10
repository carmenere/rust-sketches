SHELL := bash
SELF := $(realpath $(lastword $(MAKEFILE_LIST)))
SELFDIR := $(realpath $(dir $(SELF)))
WORKSPACE_DIR := $(realpath $(SELFDIR)/..)

CARGO_TOML ?= $(WORKSPACE_DIR)/Cargo.toml
BINS ?= example-api-sqlx
CLIPPY_FORMAT ?= json
CLIPPY_REPORT ?= clippy-report.json
FEATURES ?= 
LINTS ?= 
PROFILE ?= dev
TARGET_ARCH ?= aarch64-apple-darwin
TARGET_DIR ?= $(WORKSPACE_DIR)/target
INSTALL_DIR ?= /usr/local/bin

HOST ?= localhost
PORT ?= 5432
USER_DB ?= example_api_sqlx_db
USER_NAME ?= example_api_sqlx_user
USER_PASSWORD ?= 12345

# Use 'abspath' instead 'realpath' because TARGET_DIR is not exists, but 'realpath' checks its existance
# $1:profile,$2:TARGET_DIR,$3:TARGET_ARCH
# EXAMPLE = $(call cargo_bins,dev,target,aarch64-apple-darwin)
define cargo_bins
$(eval 
ifeq ($1,dev)
x__PROFILE_DIR = debug
else
x__PROFILE_DIR = $1
endif)$2/$3/$(x__PROFILE_DIR)
endef

BINS_DIR = $(call cargo_bins,$(PROFILE),$(TARGET_DIR),$(TARGET_ARCH))

# SUDO
SUDO_BIN ?= 
SUDO_USR ?= 

# $(and ..., ..., ...) 
# - each argument is expanded, in order;
# - if an argument expands to an empty string the processing stops and the result of the expansion is the empty string;
# - if all arguments expand to a non-empty string then the result of the expansion is the expansion of the last argument;
ifneq ($(strip $(and $(SUDO_BIN),$(SUDO_USR))),)
    SUDO = $(SUDO_BIN) -u $(SUDO_USR)
else ifneq ($(strip $(SUDO_BIN)),)
    SUDO = $(SUDO_BIN)
else
    SUDO = 
endif

define LF


endef

# ENVS
# RUSTFLAGS = -C target-feature=-crt-static
DATABASE_URL = postgres://$(USER_NAME):$(USER_PASSWORD)@$(HOST):$(PORT)/$(USER_DB)
BUILD_VERSION = $(git log -1 --pretty=format:"%h")

ENVS ?= \
    RUSTFLAGS='$(RUSTFLAGS)' \
    BUILD_VERSION='$(BUILD_VERSION)' \
    DATABASE_URL='$(DATABASE_URL)'

OPT_BINS = $(foreach BIN,$(BINS), --bin $(BIN))

# OPT_PROFILE
ifeq ($(PROFILE),release)
    OPT_PROFILE = --profile release
else
    OPT_PROFILE = --profile dev
endif

# OPT_FEATURES
ifdef FEATURES
    OPT_FEATURES = --features $(FEATURES)
else
    OPT_FEATURES =
endif

# CARGO_OPTS
CARGO_OPTS ?= $(OPT_PROFILE) $(OPT_BINS) $(OPT_FEATURES) --manifest-path $(CARGO_TOML) \
    --target-dir $(TARGET_DIR) \
    --target $(TARGET_ARCH)

CARGO_TEST_OPTS ?= $(OPT_PROFILE) $(OPT_FEATURES) --manifest-path $(CARGO_TOML) \
    --target-dir $(TARGET_DIR) \
    --target $(TARGET_ARCH) \
	--workspace \
	--exclude scd

ifdef BINS
CMD_BUILD ?= $(ENVS) cargo build $(CARGO_OPTS)
CMD_CLIPPY ?= $(ENVS) cargo clippy $(CARGO_OPTS) --message-format $(CLIPPY_FORMAT) -- $(LINTS) 1>$(CLIPPY_REPORT)
CMD_CLIPPY_FIX ?= $(ENVS) cargo clippy --fix $(CARGO_OPTS)
CMD_TEST ?= $(ENVS) cargo test $(CARGO_TEST_OPTS)
CMD_CLEAN ?= $(ENVS) cargo clean --manifest-path $(CARGO_TOML)
CMD_FMT ?=  $(ENVS) cargo +nightly fmt
CMD_FMT_CHECK ?= $(CMD_FMT) -- --check
CMD_DOC ?= $(ENVS) cargo doc --no-deps --document-private-items
endif

.PHONY: all build clippy clippy-fix lint test fmt fmt-check doc install uninstall clean distclean

all: fmt clippy build test

build:
	cd $(WORKSPACE_DIR) && $(CMD_BUILD)

clippy:
	cd $(WORKSPACE_DIR) && $(CMD_CLIPPY)

clippy-fix:
	cd $(WORKSPACE_DIR) && $(CMD_CLIPPY_FIX)

lint: clippy

test:
	cd $(WORKSPACE_DIR) && $(CMD_TEST)

fmt:
	cd $(WORKSPACE_DIR) && $(CMD_FMT)

fmt-check:
	cd $(WORKSPACE_DIR) && $(CMD_FMT_CHECK)

doc:
	cd $(WORKSPACE_DIR) && $(CMD_DOC)

clean:
	cd $(WORKSPACE_DIR) && $(CMD_CLEAN)

distclean: clean

install:
	$(SUDO) install -d $(INSTALL_DIR)
	$(foreach BIN,$(BINS),$(SUDO) install -m 755 -t $(INSTALL_DIR) $(BINS_DIR)/$(BIN) $(LF))

uninstall:
	$(foreach BIN,$(BINS),$(SUDO) rm $(INSTALL_DIR)/$(BIN) $(LF))