# 🍺 Homebrew Distribution Setup Guide

This guide walks you through making your Git Repository Analyzer available via Homebrew.

## 📋 Prerequisites Checklist

- [ ] GitHub repository is public
- [ ] Repository has proper licensing (MIT license created ✅)
- [ ] Cargo.toml has all required metadata ✅
- [ ] GitHub Actions workflow for releases ✅

## 🚀 Step-by-Step Process

### 1. Push Your Code to GitHub

```bash
# Initialize git repository (if not already done)
git init
git add .
git commit -m "Initial commit: Git Repository Analyzer v0.1.0"

# Add remote repository
git remote add origin https://github.com/alexfilatov/git-analyzer.git
git branch -M main
git push -u origin main
```

### 2. Create Your First Release

```bash
# Create and push a version tag
git tag v0.1.0
git push origin v0.1.0
```

This will trigger the GitHub Actions workflow that builds binaries for:
- Linux x86_64
- macOS x86_64 (Intel)
- macOS aarch64 (Apple Silicon)

### 3. Update the Homebrew Formula

After the release is created, you need to:

1. **Download the release assets** and calculate their SHA256 hashes:
```bash
# Download each release asset
curl -L -o git-analyzer-macos-aarch64.tar.gz \
  https://github.com/alexfilatov/git-analyzer/releases/download/v0.1.0/git-analyzer-macos-aarch64.tar.gz

curl -L -o git-analyzer-macos-x86_64.tar.gz \
  https://github.com/alexfilatov/git-analyzer/releases/download/v0.1.0/git-analyzer-macos-x86_64.tar.gz

curl -L -o git-analyzer-linux-x86_64.tar.gz \
  https://github.com/alexfilatov/git-analyzer/releases/download/v0.1.0/git-analyzer-linux-x86_64.tar.gz

# Calculate SHA256 hashes
shasum -a 256 git-analyzer-macos-aarch64.tar.gz
shasum -a 256 git-analyzer-macos-x86_64.tar.gz  
shasum -a 256 git-analyzer-linux-x86_64.tar.gz
```

2. **Update the Formula/git-analyzer.rb file** with the actual SHA256 hashes.

### 4. Distribution Options

You have several options for distributing via Homebrew:

#### Option A: Personal Tap (Recommended for start)

Create your own Homebrew tap:

```bash
# Create a new repository for your tap
# Repository name MUST be homebrew-<tap-name>
# Example: homebrew-tools

# Clone your tap repository
git clone https://github.com/alexfilatov/homebrew-tools.git
cd homebrew-tools

# Copy the formula
cp ../git-analyzer/Formula/git-analyzer.rb Formula/

# Commit and push
git add Formula/git-analyzer.rb
git commit -m "Add git-analyzer formula"
git push origin main
```

Users can then install with:
```bash
brew tap alexfilatov/tools
brew install git-analyzer
```

#### Option B: Submit to Official Homebrew (Advanced)

For inclusion in the main Homebrew repository:

1. Your tool must be notable and have sufficient usage
2. Create a pull request to [homebrew-core](https://github.com/Homebrew/homebrew-core)
3. Follow [Homebrew's contribution guidelines](https://docs.brew.sh/How-To-Open-a-Homebrew-Pull-Request)

### 5. Test Your Formula

```bash
# Test installation from your tap
brew tap alexfilatov/tools
brew install git-analyzer

# Test the binary
git-analyzer --help
git-analyzer contributors --url https://github.com/octocat/Hello-World.git

# Test uninstallation
brew uninstall git-analyzer
brew untap alexfilatov/tools
```

## 📁 Final Repository Structure

Your repository should look like this:

```
git-analyzer/
├── .github/
│   └── workflows/
│       └── release.yml          # ✅ GitHub Actions for releases
├── Formula/
│   └── git-analyzer.rb         # ✅ Homebrew formula
├── src/
│   └── main.rs                 # Your Rust code
├── Cargo.toml                  # ✅ Updated with metadata
├── LICENSE                     # ✅ MIT license
├── README.md                   # ✅ Comprehensive documentation
├── HOMEBREW_SETUP.md           # This guide
└── .gitignore
```

## 🔄 Updating Your Formula

When you release new versions:

1. **Create a new release**:
```bash
git tag v0.2.0
git push origin v0.2.0
```

2. **Update the formula** with new version, URLs, and SHA256 hashes

3. **Push to your tap repository**

## 🛠️ Advanced Features

### Automated Formula Updates

You can automate formula updates using GitHub Actions in your tap repository:

```yaml
name: Update Formula
on:
  repository_dispatch:
    types: [new-release]

jobs:
  update-formula:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Update git-analyzer formula
        run: |
          # Script to automatically update formula with new release info
          # This requires additional scripting
```

### Cask Alternative (GUI Applications)

If you ever create a GUI version, you'd use a Cask instead:

```ruby
cask "git-analyzer-gui" do
  version "0.1.0"
  sha256 "..."
  
  url "https://github.com/alexfilatov/git-analyzer/releases/download/v#{version}/GitAnalyzer.dmg"
  name "Git Analyzer"
  desc "GUI for Git repository analysis"
  homepage "https://github.com/alexfilatov/git-analyzer"
  
  app "Git Analyzer.app"
end
```

## 🎯 Next Steps

1. **Push your code** to GitHub
2. **Create the first release** (v0.1.0)
3. **Wait for GitHub Actions** to build binaries
4. **Calculate SHA256 hashes** and update the formula
5. **Create your homebrew tap** repository
6. **Test the installation** thoroughly
7. **Announce to the community**!

## 📚 Resources

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [GitHub Actions for Rust](https://github.com/actions-rs)
- [Semantic Versioning](https://semver.org/)
- [Creating Homebrew Taps](https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap)

---

**🎉 Once set up, users can install your tool with just:**
```bash
brew tap alexfilatov/tools
brew install git-analyzer
```