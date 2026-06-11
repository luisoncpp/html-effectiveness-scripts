---
title: Code Map Demo
layout: wide
---

# Build Pipeline

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
    height: 280
  - label: Pipeline
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
          this.[[run]]();
        } catch (error) {
          console.error(error.message);
          process.exit(1);
        }
      }
  - id: run
    x: 432
    y: 110
    width: 340
    language: ts
    code: |
      async [[run]](): Promise<void> {
        // Resolve the build inputs
        const args = this.[[parseArgs]](
          process.argv.slice(2)
        );

        // Load and merge configuration
        const config = await this.[[loadConfig]](
          args.configPath
        );

        // Run the compiler
        await this.[[execute]](config, args);
      }
  - id: parseArgs
    x: 880
    y: 28
    width: 330
    language: ts
    code: |
      [[parseArgs]](argv: string[]): Args {
        const args = new Args();
        for (const token of argv) {
          args.add(token);
        }
        return args;
  - id: loadConfig
    x: 880
    y: 300
    width: 330
    language: ts
    code: |
      async [[loadConfig]](
        path: string
      ): Promise<Config> {
        const raw = await readFile(path);
        return Config.parse(raw);
  - id: execute
    x: 880
    y: 540
    width: 330
    language: ts
    code: |
      async [[execute]](
        config: Config,
        args: Args
      ): Promise<void> {
        const compiler = new Compiler(config);
        await compiler.build(args.entry);
arrows:
  - from: main.run
    to: run.run
  - from: run.parseArgs
    to: parseArgs
  - from: run.loadConfig
    to: loadConfig
  - from: run.execute
    to: execute
```
