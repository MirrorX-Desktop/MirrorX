import Cocoa
import FlutterMacOS

class MainFlutterWindow: NSWindow {
    override func awakeFromNib() {
        let flutterViewController = FlutterViewController.init()
        self.contentViewController = flutterViewController
        
        self.styleMask = [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView]
        self.titleVisibility = .hidden
        self.titlebarAppearsTransparent = true
        self.isMovableByWindowBackground = true
        
        self.setFrame(NSRect(x:0, y:0, width: 995, height: 636), display: true)
        self.minSize = CGSize(width: 995, height: 636)
        self.center()
        
        RegisterGeneratedPlugins(registry: flutterViewController)
        
        super.awakeFromNib()
    }
}
