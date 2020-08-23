#!/bin/bash

# SPDX-FileCopyrightText: 2020 Wyatt Calandro <arm1stice@arm1stice.com>
#
# SPDX-License-Identifier: Apache-2.0

find .github/workflows/ src/ Cargo.toml -type f -print0 | while read -r -d $'\0' f
do
    reuse addheader --copyright "Wyatt Calandro <arm1stice@arm1stice.com>" --license Apache-2.0 "$f"
done
