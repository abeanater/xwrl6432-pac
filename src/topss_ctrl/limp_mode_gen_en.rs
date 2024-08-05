#[doc = "Register `LIMP_MODE_GEN_EN` reader"]
pub type R = crate::R<LimpModeGenEnSpec>;
#[doc = "Register `LIMP_MODE_GEN_EN` writer"]
pub type W = crate::W<LimpModeGenEnSpec>;
#[doc = "Field `dcc_error_limp_gen_enable` reader - 2:0\\]
Enable EDCC Error to generate Limp mode 3'b000: EDCC Error will not generate Limp mode (multibit 000) 3'b111 : EDCC Error will generate Limp mode (multibit 111)"]
pub type DccErrorLimpGenEnableR = crate::FieldReader;
#[doc = "Field `dcc_error_limp_gen_enable` writer - 2:0\\]
Enable EDCC Error to generate Limp mode 3'b000: EDCC Error will not generate Limp mode (multibit 000) 3'b111 : EDCC Error will generate Limp mode (multibit 111)"]
pub type DccErrorLimpGenEnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `plldig_lockmon_limp_gen_enable` reader - 10:8\\]
Enable PLLDIG lockmon to generate Limp mode 3'b000: PLLDIG lockmon will not generate Limp mode (multibit 000) 3'b111 : PLLDIG lockmon will generate Limp mode (multibit 111)"]
pub type PlldigLockmonLimpGenEnableR = crate::FieldReader;
#[doc = "Field `plldig_lockmon_limp_gen_enable` writer - 10:8\\]
Enable PLLDIG lockmon to generate Limp mode 3'b000: PLLDIG lockmon will not generate Limp mode (multibit 000) 3'b111 : PLLDIG lockmon will generate Limp mode (multibit 111)"]
pub type PlldigLockmonLimpGenEnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLDIG_LOCKMON_ESM_ERR_GEN_ENABLE` reader - 13:11\\]
Enable PLLDIG lockmon to generate esm error 3'b000: PLLDIG lockmon will not generate esm error (multibit 000) 3'b111 : PLLDIG lockmon will generate esm error (multibit 111)"]
pub type PlldigLockmonEsmErrGenEnableR = crate::FieldReader;
#[doc = "Field `PLLDIG_LOCKMON_ESM_ERR_GEN_ENABLE` writer - 13:11\\]
Enable PLLDIG lockmon to generate esm error 3'b000: PLLDIG lockmon will not generate esm error (multibit 000) 3'b111 : PLLDIG lockmon will generate esm error (multibit 111)"]
pub type PlldigLockmonEsmErrGenEnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `limp_mode_dcc` reader - 16:16\\]
status reg for limp_mode_dcc"]
pub type LimpModeDccR = crate::BitReader;
#[doc = "Field `limp_mode_dcc` writer - 16:16\\]
status reg for limp_mode_dcc"]
pub type LimpModeDccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `limp_mode_plldig_lockmon` reader - 24:24\\]
status reg for limp_mode_plldig_lockmon"]
pub type LimpModePlldigLockmonR = crate::BitReader;
#[doc = "Field `limp_mode_plldig_lockmon` writer - 24:24\\]
status reg for limp_mode_plldig_lockmon"]
pub type LimpModePlldigLockmonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Enable EDCC Error to generate Limp mode 3'b000: EDCC Error will not generate Limp mode (multibit 000) 3'b111 : EDCC Error will generate Limp mode (multibit 111)"]
    #[inline(always)]
    pub fn dcc_error_limp_gen_enable(&self) -> DccErrorLimpGenEnableR {
        DccErrorLimpGenEnableR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Enable PLLDIG lockmon to generate Limp mode 3'b000: PLLDIG lockmon will not generate Limp mode (multibit 000) 3'b111 : PLLDIG lockmon will generate Limp mode (multibit 111)"]
    #[inline(always)]
    pub fn plldig_lockmon_limp_gen_enable(&self) -> PlldigLockmonLimpGenEnableR {
        PlldigLockmonLimpGenEnableR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Enable PLLDIG lockmon to generate esm error 3'b000: PLLDIG lockmon will not generate esm error (multibit 000) 3'b111 : PLLDIG lockmon will generate esm error (multibit 111)"]
    #[inline(always)]
    pub fn plldig_lockmon_esm_err_gen_enable(&self) -> PlldigLockmonEsmErrGenEnableR {
        PlldigLockmonEsmErrGenEnableR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
status reg for limp_mode_dcc"]
    #[inline(always)]
    pub fn limp_mode_dcc(&self) -> LimpModeDccR {
        LimpModeDccR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
status reg for limp_mode_plldig_lockmon"]
    #[inline(always)]
    pub fn limp_mode_plldig_lockmon(&self) -> LimpModePlldigLockmonR {
        LimpModePlldigLockmonR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Enable EDCC Error to generate Limp mode 3'b000: EDCC Error will not generate Limp mode (multibit 000) 3'b111 : EDCC Error will generate Limp mode (multibit 111)"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_error_limp_gen_enable(&mut self) -> DccErrorLimpGenEnableW<LimpModeGenEnSpec> {
        DccErrorLimpGenEnableW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Enable PLLDIG lockmon to generate Limp mode 3'b000: PLLDIG lockmon will not generate Limp mode (multibit 000) 3'b111 : PLLDIG lockmon will generate Limp mode (multibit 111)"]
    #[inline(always)]
    #[must_use]
    pub fn plldig_lockmon_limp_gen_enable(
        &mut self,
    ) -> PlldigLockmonLimpGenEnableW<LimpModeGenEnSpec> {
        PlldigLockmonLimpGenEnableW::new(self, 8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Enable PLLDIG lockmon to generate esm error 3'b000: PLLDIG lockmon will not generate esm error (multibit 000) 3'b111 : PLLDIG lockmon will generate esm error (multibit 111)"]
    #[inline(always)]
    #[must_use]
    pub fn plldig_lockmon_esm_err_gen_enable(
        &mut self,
    ) -> PlldigLockmonEsmErrGenEnableW<LimpModeGenEnSpec> {
        PlldigLockmonEsmErrGenEnableW::new(self, 11)
    }
    #[doc = "Bit 16 - 16:16\\]
status reg for limp_mode_dcc"]
    #[inline(always)]
    #[must_use]
    pub fn limp_mode_dcc(&mut self) -> LimpModeDccW<LimpModeGenEnSpec> {
        LimpModeDccW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
status reg for limp_mode_plldig_lockmon"]
    #[inline(always)]
    #[must_use]
    pub fn limp_mode_plldig_lockmon(&mut self) -> LimpModePlldigLockmonW<LimpModeGenEnSpec> {
        LimpModePlldigLockmonW::new(self, 24)
    }
}
#[doc = "LIMP_MODE_GEN_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`limp_mode_gen_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limp_mode_gen_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LimpModeGenEnSpec;
impl crate::RegisterSpec for LimpModeGenEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`limp_mode_gen_en::R`](R) reader structure"]
impl crate::Readable for LimpModeGenEnSpec {}
#[doc = "`write(|w| ..)` method takes [`limp_mode_gen_en::W`](W) writer structure"]
impl crate::Writable for LimpModeGenEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIMP_MODE_GEN_EN to value 0"]
impl crate::Resettable for LimpModeGenEnSpec {
    const RESET_VALUE: u32 = 0;
}
