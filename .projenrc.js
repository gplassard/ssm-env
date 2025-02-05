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
          'aws-config': {version: '1.5', features: ['behavior-version-latest']},
          'aws-sdk-ssm': '1.64',
          'tokio': {version: '1.43', features: ['full']},
          'log': '0.4',
          'env_logger': '0.11',
          'clap': {version: '4.5', features: ['derive']},
          'tempfile': '3.16',
      }
   }
});
project.synth();
