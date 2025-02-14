#[doc = "Register `APPSS_TPCC_B_INTAGG_STATUS` reader"]
pub type R = crate::R<AppssTpccBIntaggStatusSpec>;
#[doc = "Register `APPSS_TPCC_B_INTAGG_STATUS` writer"]
pub type W = crate::W<AppssTpccBIntaggStatusSpec>;
#[doc = "Field `tpcc_b_intg` reader - 0:0\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBIntgR = crate::BitReader;
#[doc = "Field `tpcc_b_intg` writer - 0:0\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBIntgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_int0` reader - 1:1\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt0R = crate::BitReader;
#[doc = "Field `tpcc_b_int0` writer - 1:1\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_int1` reader - 2:2\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt1R = crate::BitReader;
#[doc = "Field `tpcc_b_int1` writer - 2:2\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_int2` reader - 3:3\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt2R = crate::BitReader;
#[doc = "Field `tpcc_b_int2` writer - 3:3\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_int3` reader - 4:4\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt3R = crate::BitReader;
#[doc = "Field `tpcc_b_int3` writer - 4:4\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_int4` reader - 5:5\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt4R = crate::BitReader;
#[doc = "Field `tpcc_b_int4` writer - 5:5\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_int5` reader - 6:6\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt5R = crate::BitReader;
#[doc = "Field `tpcc_b_int5` writer - 6:6\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_int6` reader - 7:7\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt6R = crate::BitReader;
#[doc = "Field `tpcc_b_int6` writer - 7:7\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_int7` reader - 8:8\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt7R = crate::BitReader;
#[doc = "Field `tpcc_b_int7` writer - 8:8\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TpccBInt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b0` reader - 16:16\\]
Status of Interrupt from TPTC A0. Set only if Interupt is unmasked in TPCC_A_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TptcB0R = crate::BitReader;
#[doc = "Field `tptc_b0` writer - 16:16\\]
Status of Interrupt from TPTC A0. Set only if Interupt is unmasked in TPCC_A_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TptcB0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b1` reader - 17:17\\]
Status of Interrupt from TPTC A0. Set only if Interupt is unmasked in TPCC_A_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TptcB1R = crate::BitReader;
#[doc = "Field `tptc_b1` writer - 17:17\\]
Status of Interrupt from TPTC A0. Set only if Interupt is unmasked in TPCC_A_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TptcB1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn tpcc_b_intg(&self) -> TpccBIntgR {
        TpccBIntgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn tpcc_b_int0(&self) -> TpccBInt0R {
        TpccBInt0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn tpcc_b_int1(&self) -> TpccBInt1R {
        TpccBInt1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn tpcc_b_int2(&self) -> TpccBInt2R {
        TpccBInt2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn tpcc_b_int3(&self) -> TpccBInt3R {
        TpccBInt3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn tpcc_b_int4(&self) -> TpccBInt4R {
        TpccBInt4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn tpcc_b_int5(&self) -> TpccBInt5R {
        TpccBInt5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn tpcc_b_int6(&self) -> TpccBInt6R {
        TpccBInt6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn tpcc_b_int7(&self) -> TpccBInt7R {
        TpccBInt7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Status of Interrupt from TPTC A0. Set only if Interupt is unmasked in TPCC_A_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn tptc_b0(&self) -> TptcB0R {
        TptcB0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Status of Interrupt from TPTC A0. Set only if Interupt is unmasked in TPCC_A_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn tptc_b1(&self) -> TptcB1R {
        TptcB1R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_intg(&mut self) -> TpccBIntgW<AppssTpccBIntaggStatusSpec> {
        TpccBIntgW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_int0(&mut self) -> TpccBInt0W<AppssTpccBIntaggStatusSpec> {
        TpccBInt0W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_int1(&mut self) -> TpccBInt1W<AppssTpccBIntaggStatusSpec> {
        TpccBInt1W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_int2(&mut self) -> TpccBInt2W<AppssTpccBIntaggStatusSpec> {
        TpccBInt2W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_int3(&mut self) -> TpccBInt3W<AppssTpccBIntaggStatusSpec> {
        TpccBInt3W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_int4(&mut self) -> TpccBInt4W<AppssTpccBIntaggStatusSpec> {
        TpccBInt4W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_int5(&mut self) -> TpccBInt5W<AppssTpccBIntaggStatusSpec> {
        TpccBInt5W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_int6(&mut self) -> TpccBInt6W<AppssTpccBIntaggStatusSpec> {
        TpccBInt6W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Status of Interrupt from TPCC_B. Set only if Interupt is unmasked in TPCC_B_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_int7(&mut self) -> TpccBInt7W<AppssTpccBIntaggStatusSpec> {
        TpccBInt7W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Status of Interrupt from TPTC A0. Set only if Interupt is unmasked in TPCC_A_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0(&mut self) -> TptcB0W<AppssTpccBIntaggStatusSpec> {
        TptcB0W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Status of Interrupt from TPTC A0. Set only if Interupt is unmasked in TPCC_A_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b1(&mut self) -> TptcB1W<AppssTpccBIntaggStatusSpec> {
        TptcB1W::new(self, 17)
    }
}
#[doc = "APPSS_TPCC_B_INTAGG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_b_intagg_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_b_intagg_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssTpccBIntaggStatusSpec;
impl crate::RegisterSpec for AppssTpccBIntaggStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_tpcc_b_intagg_status::R`](R) reader structure"]
impl crate::Readable for AppssTpccBIntaggStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_tpcc_b_intagg_status::W`](W) writer structure"]
impl crate::Writable for AppssTpccBIntaggStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_TPCC_B_INTAGG_STATUS to value 0"]
impl crate::Resettable for AppssTpccBIntaggStatusSpec {
    const RESET_VALUE: u32 = 0;
}
