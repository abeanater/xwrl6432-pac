#[doc = "Register `TZCTL_TZEINT` reader"]
pub type R = crate::R<TzctlTzeintSpec>;
#[doc = "Register `TZCTL_TZEINT` writer"]
pub type W = crate::W<TzctlTzeintSpec>;
#[doc = "Field `TZCTL_TZA` reader - 1:0\\]
When a trip event occurs the following action is taken on output EPWMxA. Which trip-zone pins can cause an event is defined in the TZSEL register. 0 High-impedance (EPWMxA = High-impedance state) 1h Force EPWMxA to a high state 2h Force EPWMxA to a low state 3h Do nothing, no action is taken on EPWMxA"]
pub type TzctlTzaR = crate::FieldReader;
#[doc = "Field `TZCTL_TZA` writer - 1:0\\]
When a trip event occurs the following action is taken on output EPWMxA. Which trip-zone pins can cause an event is defined in the TZSEL register. 0 High-impedance (EPWMxA = High-impedance state) 1h Force EPWMxA to a high state 2h Force EPWMxA to a low state 3h Do nothing, no action is taken on EPWMxA"]
pub type TzctlTzaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZCTL_TZB` reader - 3:2\\]
When a trip event occurs the following action is taken on output EPWMxB. Which trip-zone pins can cause an event is defined in the TZSEL register. 0 High-impedance (EPWMxB = High-impedance state) 1h Force EPWMxB to a high state 2h Force EPWMxB to a low state 3h Do nothing, no action is taken on EPWMxB."]
pub type TzctlTzbR = crate::FieldReader;
#[doc = "Field `TZCTL_TZB` writer - 3:2\\]
When a trip event occurs the following action is taken on output EPWMxB. Which trip-zone pins can cause an event is defined in the TZSEL register. 0 High-impedance (EPWMxB = High-impedance state) 1h Force EPWMxB to a high state 2h Force EPWMxB to a low state 3h Do nothing, no action is taken on EPWMxB."]
pub type TzctlTzbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZCTL_DCAEVT1` reader - 5:4\\]
Digital Compare Output A Event 1 Action On EPWMxA: 0 High-impedance (EPWMxA = High-impedance state) 1h Force EPWMxA to a high state. 2h Force EPWMxA to a low state. 3h Do Nothing, trip action is disabled"]
pub type TzctlDcaevt1R = crate::FieldReader;
#[doc = "Field `TZCTL_DCAEVT1` writer - 5:4\\]
Digital Compare Output A Event 1 Action On EPWMxA: 0 High-impedance (EPWMxA = High-impedance state) 1h Force EPWMxA to a high state. 2h Force EPWMxA to a low state. 3h Do Nothing, trip action is disabled"]
pub type TzctlDcaevt1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZCTL_DCAEVT2` reader - 7:6\\]
Digital Compare Output A Event 2 Action On EPWMxA: 0 High-impedance (EPWMxA = High-impedance state) 1h Force EPWMxA to a high state. 2h Force EPWMxA to a low state. 3h Do Nothing, trip action is disabled"]
pub type TzctlDcaevt2R = crate::FieldReader;
#[doc = "Field `TZCTL_DCAEVT2` writer - 7:6\\]
Digital Compare Output A Event 2 Action On EPWMxA: 0 High-impedance (EPWMxA = High-impedance state) 1h Force EPWMxA to a high state. 2h Force EPWMxA to a low state. 3h Do Nothing, trip action is disabled"]
pub type TzctlDcaevt2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZCTL_DCBEVT1` reader - 9:8\\]
Digital Compare Output B Event 1 Action On EPWMxB: 0 High-impedance (EPWMxB = High-impedance state) 1h Force EPWMxB to a high state. 2h Force EPWMxB to a low state. 3h Do Nothing, trip action is disabled"]
pub type TzctlDcbevt1R = crate::FieldReader;
#[doc = "Field `TZCTL_DCBEVT1` writer - 9:8\\]
Digital Compare Output B Event 1 Action On EPWMxB: 0 High-impedance (EPWMxB = High-impedance state) 1h Force EPWMxB to a high state. 2h Force EPWMxB to a low state. 3h Do Nothing, trip action is disabled"]
pub type TzctlDcbevt1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZCTL_DCBEVT2` reader - 11:10\\]
Digital Compare Output B Event 2 Action On EPWMxB: 0 High-impedance (EPWMxB = High-impedance state) 1h Force EPWMxB to a high state. 2h Force EPWMxB to a low state. 3h Do Nothing, trip action is disabled"]
pub type TzctlDcbevt2R = crate::FieldReader;
#[doc = "Field `TZCTL_DCBEVT2` writer - 11:10\\]
Digital Compare Output B Event 2 Action On EPWMxB: 0 High-impedance (EPWMxB = High-impedance state) 1h Force EPWMxB to a high state. 2h Force EPWMxB to a low state. 3h Do Nothing, trip action is disabled"]
pub type TzctlDcbevt2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved1` reader - 15:12\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 15:12\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Reserved2` reader - 16:16\\]
Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - 16:16\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEINT_CBC` reader - 17:17\\]
Trip-zone Cycle-by-Cycle Interrupt Enable 0 Disable cycle-by-cycle interrupt generation. 1 Enable interrupt generation; a cycle-by-cycle trip event will cause an EPWMx_TZINT VIM interrupt"]
pub type TzeintCbcR = crate::BitReader;
#[doc = "Field `TZEINT_CBC` writer - 17:17\\]
Trip-zone Cycle-by-Cycle Interrupt Enable 0 Disable cycle-by-cycle interrupt generation. 1 Enable interrupt generation; a cycle-by-cycle trip event will cause an EPWMx_TZINT VIM interrupt"]
pub type TzeintCbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEINT_OST` reader - 18:18\\]
Trip-zone One-Shot Interrupt Enable 0 Disable one-shot interrupt generation 1 Enable Interrupt generation; a one-shot trip event will cause a EPWMx_TZINT VIM interrupt"]
pub type TzeintOstR = crate::BitReader;
#[doc = "Field `TZEINT_OST` writer - 18:18\\]
Trip-zone One-Shot Interrupt Enable 0 Disable one-shot interrupt generation 1 Enable Interrupt generation; a one-shot trip event will cause a EPWMx_TZINT VIM interrupt"]
pub type TzeintOstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEINT_DCAEVT1` reader - 19:19\\]
Digital Comparator Output A Event 1 Interrupt Enable 0 Disabled 1 Enabled"]
pub type TzeintDcaevt1R = crate::BitReader;
#[doc = "Field `TZEINT_DCAEVT1` writer - 19:19\\]
Digital Comparator Output A Event 1 Interrupt Enable 0 Disabled 1 Enabled"]
pub type TzeintDcaevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEINT_DCAEVT2` reader - 20:20\\]
Digital Comparator Output A Event 2 Interrupt Enable 0 Disabled 1 Enabled"]
pub type TzeintDcaevt2R = crate::BitReader;
#[doc = "Field `TZEINT_DCAEVT2` writer - 20:20\\]
Digital Comparator Output A Event 2 Interrupt Enable 0 Disabled 1 Enabled"]
pub type TzeintDcaevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEINT_DCBEVT1` reader - 21:21\\]
Digital Comparator Output B Event 1 Interrupt Enable 0 Disabled 1 Enabled"]
pub type TzeintDcbevt1R = crate::BitReader;
#[doc = "Field `TZEINT_DCBEVT1` writer - 21:21\\]
Digital Comparator Output B Event 1 Interrupt Enable 0 Disabled 1 Enabled"]
pub type TzeintDcbevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEINT_DCBEVT2` reader - 22:22\\]
Digital Comparator Output B Event 2 Interrupt Enable 0 Disabled 1 Enabled"]
pub type TzeintDcbevt2R = crate::BitReader;
#[doc = "Field `TZEINT_DCBEVT2` writer - 22:22\\]
Digital Comparator Output B Event 2 Interrupt Enable 0 Disabled 1 Enabled"]
pub type TzeintDcbevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - 31:23\\]
Reserved"]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `Reserved3` writer - 31:23\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
When a trip event occurs the following action is taken on output EPWMxA. Which trip-zone pins can cause an event is defined in the TZSEL register. 0 High-impedance (EPWMxA = High-impedance state) 1h Force EPWMxA to a high state 2h Force EPWMxA to a low state 3h Do nothing, no action is taken on EPWMxA"]
    #[inline(always)]
    pub fn tzctl_tza(&self) -> TzctlTzaR {
        TzctlTzaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
When a trip event occurs the following action is taken on output EPWMxB. Which trip-zone pins can cause an event is defined in the TZSEL register. 0 High-impedance (EPWMxB = High-impedance state) 1h Force EPWMxB to a high state 2h Force EPWMxB to a low state 3h Do nothing, no action is taken on EPWMxB."]
    #[inline(always)]
    pub fn tzctl_tzb(&self) -> TzctlTzbR {
        TzctlTzbR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Digital Compare Output A Event 1 Action On EPWMxA: 0 High-impedance (EPWMxA = High-impedance state) 1h Force EPWMxA to a high state. 2h Force EPWMxA to a low state. 3h Do Nothing, trip action is disabled"]
    #[inline(always)]
    pub fn tzctl_dcaevt1(&self) -> TzctlDcaevt1R {
        TzctlDcaevt1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Digital Compare Output A Event 2 Action On EPWMxA: 0 High-impedance (EPWMxA = High-impedance state) 1h Force EPWMxA to a high state. 2h Force EPWMxA to a low state. 3h Do Nothing, trip action is disabled"]
    #[inline(always)]
    pub fn tzctl_dcaevt2(&self) -> TzctlDcaevt2R {
        TzctlDcaevt2R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Digital Compare Output B Event 1 Action On EPWMxB: 0 High-impedance (EPWMxB = High-impedance state) 1h Force EPWMxB to a high state. 2h Force EPWMxB to a low state. 3h Do Nothing, trip action is disabled"]
    #[inline(always)]
    pub fn tzctl_dcbevt1(&self) -> TzctlDcbevt1R {
        TzctlDcbevt1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Digital Compare Output B Event 2 Action On EPWMxB: 0 High-impedance (EPWMxB = High-impedance state) 1h Force EPWMxB to a high state. 2h Force EPWMxB to a low state. 3h Do Nothing, trip action is disabled"]
    #[inline(always)]
    pub fn tzctl_dcbevt2(&self) -> TzctlDcbevt2R {
        TzctlDcbevt2R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Trip-zone Cycle-by-Cycle Interrupt Enable 0 Disable cycle-by-cycle interrupt generation. 1 Enable interrupt generation; a cycle-by-cycle trip event will cause an EPWMx_TZINT VIM interrupt"]
    #[inline(always)]
    pub fn tzeint_cbc(&self) -> TzeintCbcR {
        TzeintCbcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Trip-zone One-Shot Interrupt Enable 0 Disable one-shot interrupt generation 1 Enable Interrupt generation; a one-shot trip event will cause a EPWMx_TZINT VIM interrupt"]
    #[inline(always)]
    pub fn tzeint_ost(&self) -> TzeintOstR {
        TzeintOstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Digital Comparator Output A Event 1 Interrupt Enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub fn tzeint_dcaevt1(&self) -> TzeintDcaevt1R {
        TzeintDcaevt1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Digital Comparator Output A Event 2 Interrupt Enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub fn tzeint_dcaevt2(&self) -> TzeintDcaevt2R {
        TzeintDcaevt2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Digital Comparator Output B Event 1 Interrupt Enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub fn tzeint_dcbevt1(&self) -> TzeintDcbevt1R {
        TzeintDcbevt1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Digital Comparator Output B Event 2 Interrupt Enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    pub fn tzeint_dcbevt2(&self) -> TzeintDcbevt2R {
        TzeintDcbevt2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
When a trip event occurs the following action is taken on output EPWMxA. Which trip-zone pins can cause an event is defined in the TZSEL register. 0 High-impedance (EPWMxA = High-impedance state) 1h Force EPWMxA to a high state 2h Force EPWMxA to a low state 3h Do nothing, no action is taken on EPWMxA"]
    #[inline(always)]
    #[must_use]
    pub fn tzctl_tza(&mut self) -> TzctlTzaW<TzctlTzeintSpec> {
        TzctlTzaW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
When a trip event occurs the following action is taken on output EPWMxB. Which trip-zone pins can cause an event is defined in the TZSEL register. 0 High-impedance (EPWMxB = High-impedance state) 1h Force EPWMxB to a high state 2h Force EPWMxB to a low state 3h Do nothing, no action is taken on EPWMxB."]
    #[inline(always)]
    #[must_use]
    pub fn tzctl_tzb(&mut self) -> TzctlTzbW<TzctlTzeintSpec> {
        TzctlTzbW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Digital Compare Output A Event 1 Action On EPWMxA: 0 High-impedance (EPWMxA = High-impedance state) 1h Force EPWMxA to a high state. 2h Force EPWMxA to a low state. 3h Do Nothing, trip action is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn tzctl_dcaevt1(&mut self) -> TzctlDcaevt1W<TzctlTzeintSpec> {
        TzctlDcaevt1W::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Digital Compare Output A Event 2 Action On EPWMxA: 0 High-impedance (EPWMxA = High-impedance state) 1h Force EPWMxA to a high state. 2h Force EPWMxA to a low state. 3h Do Nothing, trip action is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn tzctl_dcaevt2(&mut self) -> TzctlDcaevt2W<TzctlTzeintSpec> {
        TzctlDcaevt2W::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Digital Compare Output B Event 1 Action On EPWMxB: 0 High-impedance (EPWMxB = High-impedance state) 1h Force EPWMxB to a high state. 2h Force EPWMxB to a low state. 3h Do Nothing, trip action is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn tzctl_dcbevt1(&mut self) -> TzctlDcbevt1W<TzctlTzeintSpec> {
        TzctlDcbevt1W::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Digital Compare Output B Event 2 Action On EPWMxB: 0 High-impedance (EPWMxB = High-impedance state) 1h Force EPWMxB to a high state. 2h Force EPWMxB to a low state. 3h Do Nothing, trip action is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn tzctl_dcbevt2(&mut self) -> TzctlDcbevt2W<TzctlTzeintSpec> {
        TzctlDcbevt2W::new(self, 10)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<TzctlTzeintSpec> {
        Reserved1W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<TzctlTzeintSpec> {
        Reserved2W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Trip-zone Cycle-by-Cycle Interrupt Enable 0 Disable cycle-by-cycle interrupt generation. 1 Enable interrupt generation; a cycle-by-cycle trip event will cause an EPWMx_TZINT VIM interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tzeint_cbc(&mut self) -> TzeintCbcW<TzctlTzeintSpec> {
        TzeintCbcW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Trip-zone One-Shot Interrupt Enable 0 Disable one-shot interrupt generation 1 Enable Interrupt generation; a one-shot trip event will cause a EPWMx_TZINT VIM interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tzeint_ost(&mut self) -> TzeintOstW<TzctlTzeintSpec> {
        TzeintOstW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Digital Comparator Output A Event 1 Interrupt Enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tzeint_dcaevt1(&mut self) -> TzeintDcaevt1W<TzctlTzeintSpec> {
        TzeintDcaevt1W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Digital Comparator Output A Event 2 Interrupt Enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tzeint_dcaevt2(&mut self) -> TzeintDcaevt2W<TzctlTzeintSpec> {
        TzeintDcaevt2W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Digital Comparator Output B Event 1 Interrupt Enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tzeint_dcbevt1(&mut self) -> TzeintDcbevt1W<TzctlTzeintSpec> {
        TzeintDcbevt1W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Digital Comparator Output B Event 2 Interrupt Enable 0 Disabled 1 Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tzeint_dcbevt2(&mut self) -> TzeintDcbevt2W<TzctlTzeintSpec> {
        TzeintDcbevt2W::new(self, 22)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<TzctlTzeintSpec> {
        Reserved3W::new(self, 23)
    }
}
#[doc = "Trip-Zone Control Register/ Trip-Zone Enable Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzctl_tzeint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzctl_tzeint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzctlTzeintSpec;
impl crate::RegisterSpec for TzctlTzeintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzctl_tzeint::R`](R) reader structure"]
impl crate::Readable for TzctlTzeintSpec {}
#[doc = "`write(|w| ..)` method takes [`tzctl_tzeint::W`](W) writer structure"]
impl crate::Writable for TzctlTzeintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZCTL_TZEINT to value 0"]
impl crate::Resettable for TzctlTzeintSpec {
    const RESET_VALUE: u32 = 0;
}
