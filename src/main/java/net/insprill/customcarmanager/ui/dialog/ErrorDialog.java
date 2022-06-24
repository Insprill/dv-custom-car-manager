package net.insprill.customcarmanager.ui.dialog;

import javafx.event.ActionEvent;
import javafx.scene.control.Alert;
import javafx.scene.control.ButtonBar;
import javafx.scene.control.ButtonType;
import javafx.scene.control.TextArea;
import javafx.scene.layout.Region;
import javafx.scene.layout.VBox;
import javafx.scene.text.Text;
import javafx.stage.StageStyle;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.util.TextHelper;

/**
 * A dialog that lets the user know an error has occurred, or an action was invalid.
 */
public class ErrorDialog extends Alert {

    private final VBox content = new VBox();

    private ErrorDialog() {
        super(AlertType.ERROR, null, ButtonType.CLOSE);
        initStyle(StageStyle.UTILITY);
        headerTextProperty().setValue(Locale.getLine("dialog.error.title"));
    }

    /**
     * @param message Sets the message to display.
     * @apiNote Only use this if also adding an exception! Otherwise, use the dialogs {@code #setContentText} method.
     */
    private void setMessage(String message) {
        content.getChildren().add(new Text(message));
    }

    /**
     * Sets the exception shown in the dialog.
     *
     * @param ex The exception to show.
     */
    private void setException(Throwable ex) {
        String errorMessage = TextHelper.getStacktrace(ex);

        ButtonType copyToClipboard = new ButtonType(Locale.getLine("dialog.error.copy-stacktrace-to-clipboard"), ButtonBar.ButtonData.BACK_PREVIOUS);
        getDialogPane().getButtonTypes().add(0, copyToClipboard);

        TextArea errorText = new TextArea(errorMessage);
        errorText.setEditable(false);

        getDialogPane().lookupButton(copyToClipboard).addEventFilter(ActionEvent.ACTION, e -> {
            TextHelper.copyToClipboard(errorMessage);
            e.consume();
        });

        content.getChildren().add(errorText);
    }

    /**
     * Shows an error dialog.
     *
     * @param message The message to show.
     */
    public static void show(String message) {
        show(message, null);
    }

    /**
     * Shows an error dialog.
     *
     * @param ex The error to show.
     */
    public static void show(Throwable ex) {
        show(null, ex);
    }

    /**
     * Shows an error dialog.
     *
     * @param message The message to show.
     * @param ex      The error to show.
     */
    public static void show(String message, Throwable ex) {
        ErrorDialog dialog = new ErrorDialog();

        if (message != null) {
            if (ex != null) {
                dialog.setMessage(message);
            } else {
                dialog.setContentText(message);
            }
        }

        if (ex != null) {
            if (message != null) {
                Region spacer = new Region();
                spacer.setPrefHeight(10);
                dialog.content.getChildren().add(spacer);
            }
            dialog.setException(ex);
            dialog.getDialogPane().setContent(dialog.content);
        }

        dialog.showAndWait();
    }

}
