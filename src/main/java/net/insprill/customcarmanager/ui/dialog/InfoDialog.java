package net.insprill.customcarmanager.ui.dialog;

import javafx.scene.control.Alert;
import javafx.scene.control.ButtonType;
import javafx.stage.StageStyle;

/**
 * A dialog that lets the user know a piece of information.
 */
public class InfoDialog extends Alert {

    private InfoDialog() {
        super(AlertType.INFORMATION, null, ButtonType.CLOSE);
        initStyle(StageStyle.UTILITY);
        headerTextProperty().setValue(null);
    }

    /**
     * Shows an info dialog with the specified message. Will <i>not</i> block the thread it's called on.
     *
     * @param message The message to show.
     */
    public static void show(String message) {
        show(message, false);
    }

    /**
     * Shows an info dialog with the specified message.
     *
     * @param message  The message to show.
     * @param blocking Whether to block the thread until closed.
     */
    public static void show(String message, boolean blocking) {
        InfoDialog dialog = new InfoDialog();

        dialog.setContentText(message);

        if (blocking) {
            dialog.showAndWait();
        } else {
            dialog.show();
        }
    }

}
