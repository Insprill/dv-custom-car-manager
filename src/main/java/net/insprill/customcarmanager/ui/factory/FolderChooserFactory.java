package net.insprill.customcarmanager.ui.factory;

import javafx.stage.DirectoryChooser;
import javafx.stage.Stage;

import java.io.File;

public class FolderChooserFactory {

    public static File newDialog(Stage stage, String title) {
        DirectoryChooser fileChooser = new DirectoryChooser();

        fileChooser.setTitle(title);

        return fileChooser.showDialog(stage);
    }

}
