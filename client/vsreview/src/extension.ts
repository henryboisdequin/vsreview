import * as vscode from "vscode";
import { SidebarProvider } from "./SidebarProvider";

export function activate(context: vscode.ExtensionContext) {
  const sidebarProvider = new SidebarProvider(context.extensionUri);
  context.subscriptions.push(
    vscode.commands.registerCommand("vsreview.helloWorld", () => {
      vscode.window.showInformationMessage("Hello World from VSReview!");
    })
  );

  context.subscriptions.push(
    vscode.window.registerWebviewViewProvider("sidebar", sidebarProvider)
  );
}

export function deactivate() {}
