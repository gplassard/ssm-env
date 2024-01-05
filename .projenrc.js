const { RustProject } = require('@gplassard/projen-extensions');
const package = require('./package.json');

const project = new RustProject({
   name: 'ssm-env',
   cargo: {
      package: {
         authors: ["Gabriel Plassard <gabriel.plassard@gmail.com>"],
         version: package.version,
         edition: "2021",
      },
      dependencies: {
          'aws-config': {version: '1.1.1', features: ["behavior-version-latest"]},
          'aws-sdk-ssm': '1.9.0',
          'tokio': {version: '1', features: ["full"]},
      }
   }
});
project.synth();
