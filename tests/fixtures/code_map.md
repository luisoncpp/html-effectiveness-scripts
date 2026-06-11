---
title: Code Map Demo
layout: wide
---

# Startup Flow

```yaml
type: code-map
width: 1240
height: 770
groups:
  - label: Entry Point
    variant: amber
    x: 16
    y: 10
    width: 350
    height: 245
  - label: Initialization
    variant: green
    x: 415
    y: 10
    width: 810
    height: 745
cards:
  - id: main
    x: 32
    y: 70
    width: 318
    language: ts
    code: |
      main(): void {
        try {
          this.[[startup]]();
        } catch (error) {
          console.error(error.message);
          app.exit(1);
        }
      }
  - id: startup
    x: 432
    y: 110
    width: 340
    language: ts
    code: |
      private async [[startup]](): Promise<void>
        // Set the error handler early
        setUnexpectedErrorHandler((err) =>

        // Create services
        const [instantiationService,
        ] = this.[[createServices]]();

        try {
          // Init services
          try {
            await this.[[initServices]](envir
          } catch (error) {
            this.[[handleStartupDataDirError]](
              environmentMainService,
              productService,
              error
            );
      }
  - id: createServices
    x: 880
    y: 28
    width: 330
    language: ts
    code: |
      private [[createServices]](): [
        IInstantiationService,
        IProcessEnvironment,
        IEnvironmentMainService,
        ConfigurationService,
        StateService,
        BufferLogger,
        IProductService,
        UserDataProfilesMainService
      ] {
        const services = new ServiceColle
  - id: initServices
    x: 880
    y: 340
    width: 330
    language: ts
    code: |
      private async [[initServices]](
        environmentMainService: IEnviron
        userDataProfilesMainService: Use
        configurationService: Configurat
        stateService: StateService,
        productService: IProductService
      ): Promise<void> {
        await Promises.settled<unknown>(
  - id: handleError
    x: 880
    y: 555
    width: 330
    language: ts
    code: |
      private [[handleStartupDataDirError]](
        environmentMainService: IEnviron
        productService: IProductService,
        error: NodeJS.ErrnoException
      ): void {
        if (error.code === "EACCES" || e
          const directories = coalesce([
arrows:
  - from: main.startup
    to: startup.startup
  - from: startup.createServices
    to: createServices
  - from: startup.initServices
    to: initServices
  - from: startup.handleStartupDataDirError
    to: handleError
```
