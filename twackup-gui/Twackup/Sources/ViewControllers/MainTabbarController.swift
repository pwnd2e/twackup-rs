//
//  MainTabbarController.swift
//  Twackup
//
//  Created by Daniil on 25.11.2022.
//

import SwiftUI
import UIKit

class MainTabbarController: UITabBarController, UITabBarControllerDelegate {
    let mainModel: MainModel

    private(set) lazy var buildedPackagesVC: UIViewController = {
        let model = DebsListModel(mainModel: mainModel)
        let detailVC = DatabaseDetailVC(mainModel: mainModel)
        let mainVC = DebsListVC(model: model, detail: detailVC)

        return TwoColumnsVC(primaryVC: mainVC, secondaryVC: detailVC)
    }()

    private(set) lazy var leavesPackagesVC: UIViewController = {
        let model = LeavesPackagesModel(mainModel: mainModel)
        let detailVC = DpkgDetailVC(mainModel: mainModel)
        let mainVC = LeavesListVC(model: model, detail: detailVC)

        return TwoColumnsVC(primaryVC: mainVC, secondaryVC: detailVC)
    }()

    private(set) lazy var allPackagesVC: UIViewController = {
        let model = InstalledPackagesModel(mainModel: mainModel)
        let detailVC = DpkgDetailVC(mainModel: mainModel)
        let mainVC = DpkgListVC(model: model, detail: detailVC)

        return TwoColumnsVC(primaryVC: mainVC, secondaryVC: detailVC)
    }()

    private(set) lazy var logVC: UIViewController = {
        let logMetadata = LogVCMetadata()
        let logVC = LoggerViewController(mainModel: mainModel, metadata: logMetadata)
        let logController = SimpleNavController(rootViewController: logVC)
        logController.tabBarItem = logMetadata.tabbarItem

        return logController
    }()

    private(set) lazy var settingsVC: UIViewController = {
        let prefsMetadata = PreferencesVCMetadata()
        let rootView = SettingsViewController(mainModel: mainModel, metadata: prefsMetadata)
        let settingsController = UIHostingController(rootView: rootView)
        settingsController.tabBarItem = prefsMetadata.tabbarItem

        return settingsController
    }()

    init(mainModel: MainModel) {
        self.mainModel = mainModel

        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        delegate = self

        setViewControllers([
            buildedPackagesVC, leavesPackagesVC, allPackagesVC, logVC, settingsVC
        ], animated: false)
    }

    func tabBarController(
        _ tabBarController: UITabBarController,
        shouldSelect viewController: UIViewController
    ) -> Bool {
        // If currently on this tab - try to reset navigation stack
        if selectedViewController == viewController, let splitVC = viewController as? TwoColumnsVC {
            splitVC.resetNavigation(animated: true)
        }

        return true
    }
}
