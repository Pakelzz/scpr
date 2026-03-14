# scpr
`scpr` adalah program CLI sederhana yang saya buat untuk menampilkan jadwal sholat kota atau kabupaten di Indonesia langsung dari terminal.

Project ini dibuat untuk kebutuhan pribadi dan sebagai sarana belajar pengembangan CLI menggunakan Rust dan ekosistem Nix.

## Features
- Menampilkan jadwal sholat harian
- Mendukung kota/kabupaten di Indonesia
- Interface sederhana berbasis CLI
- Dapat diinstall melalui Nix Flakes

## Installation
Saat ini `scpr` hanya dapat diinstall melalui Nix Flake overlay.

Contoh penggunaan dalam `flake.nix`:

```nix
{ 
  inputs.scpr.url = "github:Pakelzz/scpr"; 
  outputs = { self, nixpkgs, scpr, ... }: 
  let 
    system = "x86_64-linux"; 
    pkgs = import nixpkgs { 
      inherit system; 
      overlays = [ scpr.overlays.default ]; 
    }; 
  in { 
    packages.${system}.default = pkgs.scpr; 
  }; 
}
```

## Usage
Contoh penggunaan dasar:

```bash
$ scpr surabaya
```

output contoh:
```bash
Location: KOTA SURABAYA
Date: 14/3/2026

Subuh   04:19
Dzuhur  11:42
Ashar   14:50
Magrib  17:46
Isya    18:55
```

## Status
Project ini masih dalam tahap pengembangan dan kemungkinan akan terus berubah.

## License
Personal project.