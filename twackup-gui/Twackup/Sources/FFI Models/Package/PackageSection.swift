//
//  PackageSection.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

@objc
enum PackageSection: UInt16 {
    case archiving
    case development
    case networking
    case packaging
    case system
    case terminalSupport
    case textEditors
    case themes
    case tweaks
    case utilities
    case other

    var systemImageName: String {
        switch self {
        case .archiving: return "doc.zipper"
        case .development: return "cpu"
        case .networking: return "network"
        case .packaging: return "archivebox"
        case .system: return "command"
        case .terminalSupport: return "terminal"
        case .textEditors: return "doc.text"
        case .themes: return "lasso.sparkles"
        case .tweaks: return "gearshape"
        case .utilities: return "keyboard"
        case .other: return "cube"
        }
    }

    var humanName: String {
        switch self {
        case .archiving: return "Archiving"
        case .development: return "Development"
        case .networking: return "Networking"
        case .packaging: return "Packaging"
        case .system: return "System"
        case .terminalSupport: return "Terminal support"
        case .textEditors: return "Text editors"
        case .themes: return "Themes"
        case .tweaks: return "Tweaks"
        case .utilities: return "Utilities"
        case .other: return "Other"
        }
    }
}

extension TwPackageSection_t {
    var swiftSection: PackageSection {
        switch self {
        case TwPackageSection_t(TW_PACKAGE_SECTION_ARCHIVING): return .archiving
        case TwPackageSection_t(TW_PACKAGE_SECTION_DEVELOPMENT): return .development
        case TwPackageSection_t(TW_PACKAGE_SECTION_NETWORKING): return .networking
        case TwPackageSection_t(TW_PACKAGE_SECTION_PACKAGING): return .packaging
        case TwPackageSection_t(TW_PACKAGE_SECTION_SYSTEM): return .system
        case TwPackageSection_t(TW_PACKAGE_SECTION_TERMINAL_SUPPORT): return .terminalSupport
        case TwPackageSection_t(TW_PACKAGE_SECTION_TEXT_EDITORS): return .textEditors
        case TwPackageSection_t(TW_PACKAGE_SECTION_THEMES): return .themes
        case TwPackageSection_t(TW_PACKAGE_SECTION_TWEAKS): return .tweaks
        case TwPackageSection_t(TW_PACKAGE_SECTION_UTILITIES): return .utilities

        default: return .other
        }
    }
}
