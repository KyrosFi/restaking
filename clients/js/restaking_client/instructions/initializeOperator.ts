/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type ReadonlySignerAccount,
  type TransactionSigner,
  type WritableAccount,
  type WritableSignerAccount,
} from '@solana/web3.js';
import { JITO_RESTAKING_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const INITIALIZE_OPERATOR_DISCRIMINATOR = 2;

export function getInitializeOperatorDiscriminatorBytes() {
  return getU8Encoder().encode(INITIALIZE_OPERATOR_DISCRIMINATOR);
}

export type InitializeOperatorInstruction<
  TProgram extends string = typeof JITO_RESTAKING_PROGRAM_ADDRESS,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TAccountOperator extends string | IAccountMeta<string> = string,
  TAccountAdmin extends string | IAccountMeta<string> = string,
  TAccountBase extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountConfig extends string
        ? WritableAccount<TAccountConfig>
        : TAccountConfig,
      TAccountOperator extends string
        ? WritableAccount<TAccountOperator>
        : TAccountOperator,
      TAccountAdmin extends string
        ? WritableSignerAccount<TAccountAdmin> &
            IAccountSignerMeta<TAccountAdmin>
        : TAccountAdmin,
      TAccountBase extends string
        ? ReadonlySignerAccount<TAccountBase> & IAccountSignerMeta<TAccountBase>
        : TAccountBase,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type InitializeOperatorInstructionData = { discriminator: number };

export type InitializeOperatorInstructionDataArgs = {};

export function getInitializeOperatorInstructionDataEncoder(): Encoder<InitializeOperatorInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({ ...value, discriminator: INITIALIZE_OPERATOR_DISCRIMINATOR })
  );
}

export function getInitializeOperatorInstructionDataDecoder(): Decoder<InitializeOperatorInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getInitializeOperatorInstructionDataCodec(): Codec<
  InitializeOperatorInstructionDataArgs,
  InitializeOperatorInstructionData
> {
  return combineCodec(
    getInitializeOperatorInstructionDataEncoder(),
    getInitializeOperatorInstructionDataDecoder()
  );
}

export type InitializeOperatorInput<
  TAccountConfig extends string = string,
  TAccountOperator extends string = string,
  TAccountAdmin extends string = string,
  TAccountBase extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  config: Address<TAccountConfig>;
  operator: Address<TAccountOperator>;
  admin: TransactionSigner<TAccountAdmin>;
  base: TransactionSigner<TAccountBase>;
  systemProgram?: Address<TAccountSystemProgram>;
};

export function getInitializeOperatorInstruction<
  TAccountConfig extends string,
  TAccountOperator extends string,
  TAccountAdmin extends string,
  TAccountBase extends string,
  TAccountSystemProgram extends string,
>(
  input: InitializeOperatorInput<
    TAccountConfig,
    TAccountOperator,
    TAccountAdmin,
    TAccountBase,
    TAccountSystemProgram
  >
): InitializeOperatorInstruction<
  typeof JITO_RESTAKING_PROGRAM_ADDRESS,
  TAccountConfig,
  TAccountOperator,
  TAccountAdmin,
  TAccountBase,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress = JITO_RESTAKING_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    config: { value: input.config ?? null, isWritable: true },
    operator: { value: input.operator ?? null, isWritable: true },
    admin: { value: input.admin ?? null, isWritable: true },
    base: { value: input.base ?? null, isWritable: false },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.config),
      getAccountMeta(accounts.operator),
      getAccountMeta(accounts.admin),
      getAccountMeta(accounts.base),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getInitializeOperatorInstructionDataEncoder().encode({}),
  } as InitializeOperatorInstruction<
    typeof JITO_RESTAKING_PROGRAM_ADDRESS,
    TAccountConfig,
    TAccountOperator,
    TAccountAdmin,
    TAccountBase,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedInitializeOperatorInstruction<
  TProgram extends string = typeof JITO_RESTAKING_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    config: TAccountMetas[0];
    operator: TAccountMetas[1];
    admin: TAccountMetas[2];
    base: TAccountMetas[3];
    systemProgram: TAccountMetas[4];
  };
  data: InitializeOperatorInstructionData;
};

export function parseInitializeOperatorInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedInitializeOperatorInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 5) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      config: getNextAccount(),
      operator: getNextAccount(),
      admin: getNextAccount(),
      base: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getInitializeOperatorInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
