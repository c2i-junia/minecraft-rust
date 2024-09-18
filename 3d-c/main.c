#include "raylib.h"

int main(void)
{
    // Dimensions de la fenêtre
    const int screenWidth = 800;
    const int screenHeight = 600;

    // Initialisation de la fenêtre
    InitWindow(screenWidth, screenHeight, "Mini-Jeu 3D");

    // Configuration de la caméra
    Camera camera = { 0 };
    camera.position = (Vector3){ 0.0f, 10.0f, 10.0f };
    camera.target = (Vector3){ 0.0f, 0.0f, 0.0f };
    camera.up = (Vector3){ 0.0f, 1.0f, 0.0f };
    camera.fovy = 45.0f;
    camera.projection = CAMERA_PERSPECTIVE;

    SetTargetFPS(60); // Limite à 60 FPS

    // Position du joueur
    Vector3 playerPosition = { 0.0f, 1.0f, 0.0f };
    float playerSize = 1.0f;

    // Taille de la plateforme
    Vector3 platformSize = { 20.0f, 1.0f, 20.0f };

    // Vitesse de déplacement
    float playerSpeed = 5.0f;

    // Boucle principale du jeu
    while (!WindowShouldClose())
    {
        // Gestion des entrées
        if (IsKeyDown(KEY_W)) playerPosition.z += playerSpeed * GetFrameTime();
        if (IsKeyDown(KEY_S)) playerPosition.z -= playerSpeed * GetFrameTime();
        if (IsKeyDown(KEY_A)) playerPosition.x += playerSpeed * GetFrameTime();
        if (IsKeyDown(KEY_D)) playerPosition.x -= playerSpeed * GetFrameTime();

        BeginDrawing();

            ClearBackground(RAYWHITE);

            BeginMode3D(camera);

                // Dessiner la plateforme
                DrawCube((Vector3){0.0f, 0.0f, 0.0f}, platformSize.x, platformSize.y, platformSize.z, LIGHTGRAY);
                DrawCubeWires((Vector3){0.0f, 0.0f, 0.0f}, platformSize.x, platformSize.y, platformSize.z, DARKGRAY);

                // Dessiner le joueur
                DrawCube(playerPosition, playerSize, playerSize, playerSize, RED);
                DrawCubeWires(playerPosition, playerSize, playerSize, playerSize, MAROON);

            EndMode3D();

            DrawFPS(10, 10);

        EndDrawing();
    }

    // Déchargement des ressources
    CloseWindow();

    return 0;
}
