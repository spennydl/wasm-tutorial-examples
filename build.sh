#!/usr/bin/env bash

#
# Companion example to the bottoms-up wasm guide.
# Copyright (C) 2019  Spencer Leslie <spencerdleslie@gmail.com>
#
# This program is free software; you can redistribute it and/or
# modify it under the terms of the GNU General Public License
# as published by the Free Software Foundation; either version 2
# of the License, or (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program; if not, write to the Free Software
# Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
#

#######
# functions #
       ######
# Usage and example
usage() {
    echo "Usage: ./build.sh [-p|--project {PROJECT_NAME}]"
    echo ""
    echo "  Builds examples in the project and deploys them to the corresponding project's /public directories."
    echo "  By default, all projects are built."
    echo "  Use the --project switch to specify a single project to build."
    echo ""
}

# deploy a wasm module
deploy_single() {
    local project_name="$1"
    local module_name="$(echo "$project_name" | sed s/-/_/g)"
    echo "Deploying $module_name to ${project_name}/public..."
    cp "target/wasm32-unknown-unknown/debug/${module_name}.wasm" "${project_name}/public"
}

# build and deploy a single project
build_single() {
    local project_name="$1"
    if [[ "$project_name" == "" ]]; then
        usage
        exit 1
    fi

    echo "Building $project_name..."
    set -e
    cargo build -p "$project_name" --target wasm32-unknown-unknown
    deploy_single "$project_name"
}

# build and deploy all projects
build_all() {
    local project_names=("part1-broken-alert" "part1-custom-alert" "part1-working-alert" "part1-julia-set")

    echo "Building all..."
    cargo build --target wasm32-unknown-unknown

    for project_name in "${project_names[@]}"; do
        deploy_single "$project_name"
    done
}

########
# main begins #
         ######

# parse args
while [[ $# -gt 0 ]]; do
    opt="$1"
    shift;
    case "$opt" in
        "-p"|"--project"           ) project="$1"; proj_specified=1; shift;;
        "-h"|"--help"              ) usage
                         exit 0;;
        *                          ) echo "ERROR: Invalid option: \""$opt"\"" >&2
                         exit 1;;
    esac
done

# check for the wasm32-unknown-unknown target
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "Error: you must have the wasm32-unkonwn-unknown target installed!"
    echo "Run ""rustup target add wasm32-unkonwn-unknown"" to add this target"
    exit 1
fi

# build
if [[ "$proj_specified" -eq 1 ]]; then
    build_single "$project"
else
    build_all
fi

