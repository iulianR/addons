module Main exposing (Model)

import Array exposing (Array)
import Browser
import Browser.Navigation as Nav
import Debug exposing (log, toString)
import Html exposing (Html, button, div, li, text, ul)
import Html.Events exposing (onClick)
import Http
import Json.Decode exposing (Decoder, field, int, map)
import Page.Register as Register
import Route exposing (Route)
import Url



-- MAIN


main : Program () Model Msg
main =
    Browser.application { init = init, update = update, view = view, subscriptions = subscriptions, onUrlChange = UrlChanged, onUrlRequest = LinkClicked }


type Model
    = Register Register.Model
    | Home



-- MODEL


type alias Addon =
    { name : String
    }


type Msg
    = NoOp
    | LinkClicked Browser.UrlRequest
    | UrlChanged Url.Url
      -- | GotAddons (Result Http.Error (List Addon))
    | GotRegisterMsg Register.Msg



-- INIT


init : () -> Url.Url -> Nav.Key -> ( Model, Cmd Msg )
init _ url key =
    changeRouteTo  key (Route.fromUrl url) Home



-- UPDATE


changeRouteTo : Nav.Key -> Maybe Route -> Model -> ( Model, Cmd Msg )
changeRouteTo navKey maybeRoute model =
    case maybeRoute of
        Nothing ->
            ( Home, Cmd.none )

        Just Route.Home ->
            ( model, Route.replaceUrl navKey Route.Home )

        Just Route.Register ->
            Register.init
                |> updateWith Register GotRegisterMsg model


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case ( message, model ) of
        ( NoOp, _ ) ->
            ( model, Cmd.none )

        ( LinkClicked request, _ ) ->
            case request of
                Browser.Internal url ->
                    case url.fragment of
                        Nothing ->
                            -- If we got a link that didn't include a fragment,
                            -- it's from one of those (href "") attributes that
                            -- we have to include to make the RealWorld CSS work.
                            --
                            -- In an application doing path routing instead of
                            -- fragment-based routing, this entire
                            -- `case url.fragment of` expression this comment
                            -- is inside would be unnecessary.
                            ( model, Cmd.none )

                        Just _ ->
                            ( model
                            , Nav.pushUrl (Session.navKey (toSession model)) (Url.toString url)
                            )

        ( UrlChanged url, _ ) ->
            ( model, Cmd.none )

        -- ( GotAddons result, _ ) ->
        -- case result of
        --     Ok addons ->
        --         ( { model | addons = addons }, Cmd.none )
        --     Err _ ->
        -- ( model, Cmd.none )
        ( GotRegisterMsg msg, Register register ) ->
            Register.update msg register
                |> updateWith Register GotRegisterMsg model

        ( _, _ ) ->
            ( model, Cmd.none )


updateWith : (subModel -> Model) -> (subMsg -> Msg) -> Model -> ( subModel, Cmd subMsg ) -> ( Model, Cmd Msg )
updateWith toModel toMsg model ( subModel, subCmd ) =
    ( toModel subModel
    , Cmd.map toMsg subCmd
    )



-- VIEW


view : Model -> Browser.Document Msg
view model =
    case model of
        Home ->
            { title = "Home"
            , body = []
            }

        Register register ->
            -- let body =
            { title = "Hello World!"
            , body = List.map (Html.map GotRegisterMsg) [ Register.view register ]
            }


viewAddon : { a | name : String } -> Html msg
viewAddon addon =
    li [] [ text addon.name ]



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none



-- HTTP
-- fetchAddons : Cmd Msg
-- fetchAddons =
--     Http.request
--         { method = "GET"
--         , headers =
--             [-- Http.header "Access-Control-Allow-Origin" "*"
--              -- Http.header "User-Agent" "ElmApp"
--             ]
--         , url = "http://127.0.0.1:3030/addons/"
--         , body = Http.emptyBody
--         , expect = Http.expectJson GotAddons addonsDecoder
--         , timeout = Nothing
--         , tracker = Nothing
--         }


addonsDecoder : Decoder (List Addon)
addonsDecoder =
    Json.Decode.list addonDecoder


addonDecoder : Decoder Addon
addonDecoder =
    map Addon
        (Json.Decode.field "name" Json.Decode.string)
