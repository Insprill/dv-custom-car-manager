package net.insprill.customcarmanager.ui.factory;

import javafx.stage.DirectoryChooser;
import net.insprill.customcarmanager.ui.Window;

import java.io.File;

public class FolderChooserFactory {

    public static File newDialog(String title) {
        DirectoryChooser fileChooser = new DirectoryChooser();

        fileChooser.setTitle(title);

        return fileChooser.showDialog(Window.getInstance().getPrimaryStage());
    }

}
