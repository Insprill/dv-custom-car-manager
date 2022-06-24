package net.insprill.customcarmanager.ui.dialog;

import javafx.scene.control.Alert;
import javafx.scene.control.Button;
import javafx.scene.control.ButtonType;
import javafx.stage.StageStyle;

import java.util.Optional;

/**
 * A dialog that lets the user choose whether to continue with an action.
 */
public class ConfirmationDialog extends Alert {

    private ConfirmationDialog(String message) {
        super(AlertType.CONFIRMATION, null, ButtonType.YES, ButtonType.NO);
        initStyle(StageStyle.UTILITY);
        headerTextProperty().setValue(null);
        setContentText(message);
    }

    /**
     * Shows the confirmation dialog, with the "Yes" button being highlighted by default.
     *
     * @param message The message to show.
     * @return True if the user selected "Yes", false otherwise.
     */
    public static boolean show(String message) {
        return ConfirmationDialog.show(message, false);
    }

    /**
     * Shows the confirmation dialog, with the "Yes".
     *
     * @param message      The message to show.
     * @param isYesDefault Whether the "Yes" button should be highlighted by default, or the "No" button.
     * @return True if the user selected "Yes", false otherwise.
     */
    public static boolean show(String message, boolean isYesDefault) {
        ConfirmationDialog dialog = new ConfirmationDialog(message);
        ((Button) dialog.getDialogPane().lookupButton(ButtonType.YES)).setDefaultButton(isYesDefault);
        ((Button) dialog.getDialogPane().lookupButton(ButtonType.NO)).setDefaultButton(!isYesDefault);
        Optional<ButtonType> result = dialog.showAndWait();
        return result.filter(buttonType -> buttonType == ButtonType.YES).isPresent();
    }

}
