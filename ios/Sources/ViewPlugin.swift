import Tauri
import WebKit

class ViewArgs: Decodable {
  let path: String?
}

class ViewPlugin: Plugin {
  @objc public func view(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(ViewArgs.self)

    DispatchQueue.main.async {
      let controller = Controller()
      controller.setUrl(URL(fileURLWithPath: args.path ?? ""))
      self.manager.viewController?.addChild(controller)
      self.manager.viewController?.view.addSubview(controller.view)
      invoke.resolve(["value": "OK"])
    }
  }
}

@_cdecl("init_plugin_view")
func initPlugin() -> Plugin {
  return ViewPlugin()
}

class Controller: UIViewController, UIDocumentInteractionControllerDelegate {
  var url: URL!
  var controller: UIDocumentInteractionController!

  func setUrl(_ url: URL) {
    self.url = url
  }

  override func viewDidLoad() {
    super.viewDidLoad()
    controller = UIDocumentInteractionController(url: self.url)
    controller.delegate = self
    if !controller.presentPreview(animated: true) {
      controller.presentOpenInMenu(from: self.view.bounds, in: self.view, animated: true)
    }
  }

  func documentInteractionControllerDidEndPreview(_ controller: UIDocumentInteractionController) {
    self.view.removeFromSuperview()
    self.removeFromParent()
  }

  func documentInteractionControllerDidDismissOpenInMenu(
    _ controller: UIDocumentInteractionController
  ) {
    self.view.removeFromSuperview()
    self.removeFromParent()
  }

  func documentInteractionControllerViewControllerForPreview(
    _ controller: UIDocumentInteractionController
  ) -> UIViewController {
    return self
  }
}
