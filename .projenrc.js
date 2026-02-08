const { RustProject } = require('@gplassard/projen-extensions');
const package = require('./package.json');

const project = new RustProject({
   name: 'ssm-env',
   cargo: {
      package: {
         authors: ['Gabriel Plassard <gabriel.plassard@gmail.com>'],
         version: package.version,
         edition: '2021',
      },
      dependencies: {
          'aws-config': {version: '1.8', features: ['behavior-version-latest']},
          'aws-sdk-ssm': '1.91',
          'tokio': {version: '1.47', features: ['full']},
          'log': '0.4',
          'env_logger': '0.11',
          'clap': {version: '4.5', features: ['derive']},
          'tempfile': '3.21',
      }
   }
});
project.tryFindObjectFile('release-please-config.json')?.addOverride('packages.\\..include-component-in-tag', false)
project.synth();
