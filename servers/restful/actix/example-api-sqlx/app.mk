SHELL := bash
SELF := $(realpath $(lastword $(MAKEFILE_LIST)))
SELFDIR := $(realpath $(dir $(SELF)))
WORKSPACE := $(realpath $(SELFDIR)/..)

APP ?= example-api-sqlx
BIN_PATH ?= $(WORKSPACE)/target/aarch64-apple-darwin/debug/example-api-sqlx
LOG_FILE ?= $(SELFDIR)/.artefacts/.logs/$(APP)
PID_FILE ?= $(SELFDIR)/.artefacts/.pid/$(APP)
PKILL_PATTERN ?= $(BIN_PATH)
MODE ?= shell

HOST ?= localhost
PORT ?= 5432
USER_DB ?= example_api_sqlx_db
USER_NAME ?= example_api_sqlx_user
USER_PASSWORD ?= 12345

MIGRATIONS = migrations
SEVERITY = debug
RUST_LOG = actix=$(SEVERITY),actix_web=$(SEVERITY),example_api_sqlx=$(SEVERITY),sqlx=$(SEVERITY)

# ENVS
ENVS ?= \
    RUST_LOG='$(RUST_LOG)' \
    PG_HOST='$(HOST)' \
    PG_PORT='$(PORT)' \
    PG_DB='$(USER_DB)' \
    PG_USER='$(USER_NAME)' \
    PG_PASSWORD='$(USER_PASSWORD)'

define escape
$(subst ",\",$(subst ',\',$1))
endef

ifdef BIN_PATH
    START_BIN ?= $(ENVS) $(BIN_PATH) $(OPTS)
else
    START_BIN ?=
endif

.PHONY: init shell tee run clean distclean

init:
	[ -d $(dir $(LOG_FILE)) ] || mkdir -p $(dir $(LOG_FILE))
	[ -d $(dir $(PID_FILE)) ] || mkdir -p $(dir $(PID_FILE))

shell:
ifdef START_BIN
	$(START_BIN)
endif

tee:
	echo ENVS = $$'$(call escape,$(ENVS))' > $(LOG_FILE)
ifdef START_BIN
	bash -c $$'$(call escape,$(START_BIN) 2>&1 | tee -a $(LOG_FILE); exit $${PIPESTATUS[0]})'
endif

run: init $(MODE)

clean:
	[ ! -f $(LOG_FILE) ] || rm -vf $(LOG_FILE)
	[ ! -f $(PID_FILE) ] || rm -vf $(PID_FILE)

distclean: clean
