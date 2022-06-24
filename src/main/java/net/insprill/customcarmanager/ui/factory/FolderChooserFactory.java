package net.insprill.customcarmanager.ui.factory;

import javafx.stage.DirectoryChooser;
import net.insprill.customcarmanager.ui.Window;

import java.io.File;

public class FolderChooserFactory {

    /**
     * Shows a new {@link DirectoryChooser} with the specified title.
     *
     * @param title The title of the file chooser.
     * @return The selected directory, or null if the window was closed/ the cancel button was pressed.
     */
    public static File newDialog(String title) {
        DirectoryChooser fileChooser = new DirectoryChooser();

        fileChooser.setTitle(title);

        return fileChooser.showDialog(Window.getInstance().getPrimaryStage());
    }

    private FolderChooserFactory() {
        throw new IllegalStateException("Utility class");
    }

}
