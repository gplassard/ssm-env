# Changelog

## [0.4.1](https://github.com/gplassard/ssm-env/compare/v0.4.0...v0.4.1) (2026-02-08)


### Features

* **ansible-vault:** add ansible-vault support ([#3](https://github.com/gplassard/ssm-env/issues/3)) ([45210df](https://github.com/gplassard/ssm-env/commit/45210df9cfebd9cdc293bd8e44089fea5cce8487))
* **cli:** better help, specify log-level and path ([#2](https://github.com/gplassard/ssm-env/issues/2)) ([b21d4ec](https://github.com/gplassard/ssm-env/commit/b21d4ec2772623733b0ca2c910885eec26d6a907))
* **cli:** retrieve command from cli args ([#1](https://github.com/gplassard/ssm-env/issues/1)) ([43c7395](https://github.com/gplassard/ssm-env/commit/43c7395cb30821a3f8cd538a70ea3c1f2b7e3673))
* **context:** add context support ([#4](https://github.com/gplassard/ssm-env/issues/4)) ([f35fb7c](https://github.com/gplassard/ssm-env/commit/f35fb7c447014a69be881dbffb28c74cc5c59be7))
* **exec:** add support to pass several contexts or path prefixes ([#21](https://github.com/gplassard/ssm-env/issues/21)) ([ff7cde7](https://github.com/gplassard/ssm-env/commit/ff7cde71670ae1b733ac385f5db8f000372fc03c))
* **parameters:** support direct mapping of SSM parameters to environ… ([#52](https://github.com/gplassard/ssm-env/issues/52)) ([fe4e550](https://github.com/gplassard/ssm-env/commit/fe4e55045c3c0f864199c308e644f499250a552e))
* **prefix:** add support for prefixing the env variables ([#29](https://github.com/gplassard/ssm-env/issues/29)) ([c62a804](https://github.com/gplassard/ssm-env/commit/c62a8042f7ed1f01de0e5fc8e0c94541b89fb681))
* **setup:** setup project ([d582abd](https://github.com/gplassard/ssm-env/commit/d582abd2900a4f3b5f6893d2c0fcb74b789ff2b1))
* **version:** add version command ([#25](https://github.com/gplassard/ssm-env/issues/25)) ([bda3614](https://github.com/gplassard/ssm-env/commit/bda3614b3291c3e876808ffcc1fb506327827827))


### Bug Fixes

* **ansible:** fix env variable for ansible mode ([#35](https://github.com/gplassard/ssm-env/issues/35)) ([a6978ee](https://github.com/gplassard/ssm-env/commit/a6978ee4507e496772c6d9d0413d005ae604cb04))
* **cross-build:** remove cross build CI ([#30](https://github.com/gplassard/ssm-env/issues/30)) ([02e5992](https://github.com/gplassard/ssm-env/commit/02e5992e6c2f82cce8d8ef992735a4f23937af0a))
* **paths:** adjust default path and improve docs ([#26](https://github.com/gplassard/ssm-env/issues/26)) ([74d45f1](https://github.com/gplassard/ssm-env/commit/74d45f1a2d2719c55e199865d9309f1a1d8f950c))
* **projen:** regen ([91cf7ee](https://github.com/gplassard/ssm-env/commit/91cf7ee9bf5833aa8637be506c2230e24c616aa3))
* **release:** add content permissions ([#11](https://github.com/gplassard/ssm-env/issues/11)) ([f68f2e2](https://github.com/gplassard/ssm-env/commit/f68f2e2d5f527f7668dca3ab6850d0ad20f9d634))
* **release:** maybe fix the release process ([06981bc](https://github.com/gplassard/ssm-env/commit/06981bced2dd5fbcc8b7a6d6246c53c93cfc1812))


### Miscellaneous Chores

* release 0.3.4 ([a871d16](https://github.com/gplassard/ssm-env/commit/a871d164994334a2515150dfab166ab8eb507fad))
* release 0.3.5 ([ad3e24e](https://github.com/gplassard/ssm-env/commit/ad3e24ee505b0883f232055011190ed28e9a49be))
* release 0.3.6 ([6028176](https://github.com/gplassard/ssm-env/commit/60281762dd7487af04a2a831827b80b81017eeec))
* release 0.3.7 ([5672c83](https://github.com/gplassard/ssm-env/commit/5672c83b5728c1dd39e90d0dd253cd845785993b))
* release 0.3.8 ([e78eef8](https://github.com/gplassard/ssm-env/commit/e78eef8e76fddf4c1ccac0812f060008acfd7405))
* release 0.4.0 ([16223bc](https://github.com/gplassard/ssm-env/commit/16223bcc20235e7e7e4548326ae25e3276c422b4))
* release 0.4.1 ([253bbcc](https://github.com/gplassard/ssm-env/commit/253bbcc3aeceaa37566c0f4b96a818e80cc5fa69))

## [0.4.0](https://github.com/gplassard/ssm-env/compare/ssm-env-v0.3.8...ssm-env-v0.4.0) (2026-02-08)


### Features

* **ansible-vault:** add ansible-vault support ([#3](https://github.com/gplassard/ssm-env/issues/3)) ([45210df](https://github.com/gplassard/ssm-env/commit/45210df9cfebd9cdc293bd8e44089fea5cce8487))
* **cli:** better help, specify log-level and path ([#2](https://github.com/gplassard/ssm-env/issues/2)) ([b21d4ec](https://github.com/gplassard/ssm-env/commit/b21d4ec2772623733b0ca2c910885eec26d6a907))
* **cli:** retrieve command from cli args ([#1](https://github.com/gplassard/ssm-env/issues/1)) ([43c7395](https://github.com/gplassard/ssm-env/commit/43c7395cb30821a3f8cd538a70ea3c1f2b7e3673))
* **context:** add context support ([#4](https://github.com/gplassard/ssm-env/issues/4)) ([f35fb7c](https://github.com/gplassard/ssm-env/commit/f35fb7c447014a69be881dbffb28c74cc5c59be7))
* **exec:** add support to pass several contexts or path prefixes ([#21](https://github.com/gplassard/ssm-env/issues/21)) ([ff7cde7](https://github.com/gplassard/ssm-env/commit/ff7cde71670ae1b733ac385f5db8f000372fc03c))
* **parameters:** support direct mapping of SSM parameters to environ… ([#52](https://github.com/gplassard/ssm-env/issues/52)) ([fe4e550](https://github.com/gplassard/ssm-env/commit/fe4e55045c3c0f864199c308e644f499250a552e))
* **prefix:** add support for prefixing the env variables ([#29](https://github.com/gplassard/ssm-env/issues/29)) ([c62a804](https://github.com/gplassard/ssm-env/commit/c62a8042f7ed1f01de0e5fc8e0c94541b89fb681))
* **setup:** setup project ([d582abd](https://github.com/gplassard/ssm-env/commit/d582abd2900a4f3b5f6893d2c0fcb74b789ff2b1))
* **version:** add version command ([#25](https://github.com/gplassard/ssm-env/issues/25)) ([bda3614](https://github.com/gplassard/ssm-env/commit/bda3614b3291c3e876808ffcc1fb506327827827))


### Bug Fixes

* **ansible:** fix env variable for ansible mode ([#35](https://github.com/gplassard/ssm-env/issues/35)) ([a6978ee](https://github.com/gplassard/ssm-env/commit/a6978ee4507e496772c6d9d0413d005ae604cb04))
* **cross-build:** remove cross build CI ([#30](https://github.com/gplassard/ssm-env/issues/30)) ([02e5992](https://github.com/gplassard/ssm-env/commit/02e5992e6c2f82cce8d8ef992735a4f23937af0a))
* **paths:** adjust default path and improve docs ([#26](https://github.com/gplassard/ssm-env/issues/26)) ([74d45f1](https://github.com/gplassard/ssm-env/commit/74d45f1a2d2719c55e199865d9309f1a1d8f950c))
* **projen:** regen ([91cf7ee](https://github.com/gplassard/ssm-env/commit/91cf7ee9bf5833aa8637be506c2230e24c616aa3))
* **release:** add content permissions ([#11](https://github.com/gplassard/ssm-env/issues/11)) ([f68f2e2](https://github.com/gplassard/ssm-env/commit/f68f2e2d5f527f7668dca3ab6850d0ad20f9d634))
* **release:** maybe fix the release process ([06981bc](https://github.com/gplassard/ssm-env/commit/06981bced2dd5fbcc8b7a6d6246c53c93cfc1812))


### Miscellaneous Chores

* release 0.3.4 ([a871d16](https://github.com/gplassard/ssm-env/commit/a871d164994334a2515150dfab166ab8eb507fad))
* release 0.3.5 ([ad3e24e](https://github.com/gplassard/ssm-env/commit/ad3e24ee505b0883f232055011190ed28e9a49be))
* release 0.3.6 ([6028176](https://github.com/gplassard/ssm-env/commit/60281762dd7487af04a2a831827b80b81017eeec))
* release 0.3.7 ([5672c83](https://github.com/gplassard/ssm-env/commit/5672c83b5728c1dd39e90d0dd253cd845785993b))
* release 0.3.8 ([e78eef8](https://github.com/gplassard/ssm-env/commit/e78eef8e76fddf4c1ccac0812f060008acfd7405))
* release 0.4.0 ([16223bc](https://github.com/gplassard/ssm-env/commit/16223bcc20235e7e7e4548326ae25e3276c422b4))

## [0.3.8](https://github.com/gplassard/ssm-env/compare/v0.3.7...v0.3.8) (2025-09-01)


### Miscellaneous Chores

* release 0.3.8 ([e78eef8](https://github.com/gplassard/ssm-env/commit/e78eef8e76fddf4c1ccac0812f060008acfd7405))

## [0.3.7](https://github.com/gplassard/ssm-env/compare/v0.3.6...v0.3.7) (2025-09-01)


### Miscellaneous Chores

* release 0.3.7 ([5672c83](https://github.com/gplassard/ssm-env/commit/5672c83b5728c1dd39e90d0dd253cd845785993b))

## [0.3.6](https://github.com/gplassard/ssm-env/compare/v0.3.5...v0.3.6) (2025-09-01)


### Miscellaneous Chores

* release 0.3.6 ([6028176](https://github.com/gplassard/ssm-env/commit/60281762dd7487af04a2a831827b80b81017eeec))

## [0.3.5](https://github.com/gplassard/ssm-env/compare/v0.3.4...v0.3.5) (2025-09-01)


### Bug Fixes

* **release:** maybe fix the release process ([06981bc](https://github.com/gplassard/ssm-env/commit/06981bced2dd5fbcc8b7a6d6246c53c93cfc1812))


### Miscellaneous Chores

* release 0.3.4 ([a871d16](https://github.com/gplassard/ssm-env/commit/a871d164994334a2515150dfab166ab8eb507fad))
* release 0.3.5 ([ad3e24e](https://github.com/gplassard/ssm-env/commit/ad3e24ee505b0883f232055011190ed28e9a49be))

## [0.3.4](https://github.com/gplassard/ssm-env/compare/v0.3.4...v0.3.4) (2025-09-01)


### Bug Fixes

* **release:** maybe fix the release process ([06981bc](https://github.com/gplassard/ssm-env/commit/06981bced2dd5fbcc8b7a6d6246c53c93cfc1812))


### Miscellaneous Chores

* release 0.3.4 ([a871d16](https://github.com/gplassard/ssm-env/commit/a871d164994334a2515150dfab166ab8eb507fad))
