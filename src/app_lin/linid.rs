#[doc = "Register `LINID` reader"]
pub type R = crate::R<LinidSpec>;
#[doc = "Register `LINID` writer"]
pub type W = crate::W<LinidSpec>;
#[doc = "Field `IDBYTE` reader - 7:0\\]
ID byte. This field is effective in LIN mode only. This byte is the LIN mode message ID. On a master node, a write to this register by the CPU initiates a header transmission. For a slave task, this byte is used for message filtering when HGENCTRL (SCIGCR1.12) is '0'. These bits are writable in LIN mode only."]
pub type IdbyteR = crate::FieldReader;
#[doc = "Field `IDBYTE` writer - 7:0\\]
ID byte. This field is effective in LIN mode only. This byte is the LIN mode message ID. On a master node, a write to this register by the CPU initiates a header transmission. For a slave task, this byte is used for message filtering when HGENCTRL (SCIGCR1.12) is '0'. These bits are writable in LIN mode only."]
pub type IdbyteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IDSLAVETASKBYTE` reader - 15:8\\]
ID Slave Task byte. This field is effective in LIN mode only. This byte contains the identifier to which the received ID of an incoming header will be compared in order to decide whether a RX response, a TX response, or no action needs to be done by the LIN node. These bits are writable in LIN mode only."]
pub type IdslavetaskbyteR = crate::FieldReader;
#[doc = "Field `IDSLAVETASKBYTE` writer - 15:8\\]
ID Slave Task byte. This field is effective in LIN mode only. This byte contains the identifier to which the received ID of an incoming header will be compared in order to decide whether a RX response, a TX response, or no action needs to be done by the LIN node. These bits are writable in LIN mode only."]
pub type IdslavetaskbyteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RECEIVEDID` reader - 23:16\\]
Received ID. This bit is effective in LIN mode only. This byte contains the current message identifier. During header reception the received ID is copied from the SCIRXSHF register to this byte if there is no ID-parity error and there has been an RX/TX match. Note: If a framing error (FE) is detected during ID reception, the received ID will also not be copied to the LINID register."]
pub type ReceivedidR = crate::FieldReader;
#[doc = "Field `RECEIVEDID` writer - 23:16\\]
Received ID. This bit is effective in LIN mode only. This byte contains the current message identifier. During header reception the received ID is copied from the SCIRXSHF register to this byte if there is no ID-parity error and there has been an RX/TX match. Note: If a framing error (FE) is detected during ID reception, the received ID will also not be copied to the LINID register."]
pub type ReceivedidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
ID byte. This field is effective in LIN mode only. This byte is the LIN mode message ID. On a master node, a write to this register by the CPU initiates a header transmission. For a slave task, this byte is used for message filtering when HGENCTRL (SCIGCR1.12) is '0'. These bits are writable in LIN mode only."]
    #[inline(always)]
    pub fn idbyte(&self) -> IdbyteR {
        IdbyteR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
ID Slave Task byte. This field is effective in LIN mode only. This byte contains the identifier to which the received ID of an incoming header will be compared in order to decide whether a RX response, a TX response, or no action needs to be done by the LIN node. These bits are writable in LIN mode only."]
    #[inline(always)]
    pub fn idslavetaskbyte(&self) -> IdslavetaskbyteR {
        IdslavetaskbyteR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Received ID. This bit is effective in LIN mode only. This byte contains the current message identifier. During header reception the received ID is copied from the SCIRXSHF register to this byte if there is no ID-parity error and there has been an RX/TX match. Note: If a framing error (FE) is detected during ID reception, the received ID will also not be copied to the LINID register."]
    #[inline(always)]
    pub fn receivedid(&self) -> ReceivedidR {
        ReceivedidR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
ID byte. This field is effective in LIN mode only. This byte is the LIN mode message ID. On a master node, a write to this register by the CPU initiates a header transmission. For a slave task, this byte is used for message filtering when HGENCTRL (SCIGCR1.12) is '0'. These bits are writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn idbyte(&mut self) -> IdbyteW<LinidSpec> {
        IdbyteW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
ID Slave Task byte. This field is effective in LIN mode only. This byte contains the identifier to which the received ID of an incoming header will be compared in order to decide whether a RX response, a TX response, or no action needs to be done by the LIN node. These bits are writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn idslavetaskbyte(&mut self) -> IdslavetaskbyteW<LinidSpec> {
        IdslavetaskbyteW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Received ID. This bit is effective in LIN mode only. This byte contains the current message identifier. During header reception the received ID is copied from the SCIRXSHF register to this byte if there is no ID-parity error and there has been an RX/TX match. Note: If a framing error (FE) is detected during ID reception, the received ID will also not be copied to the LINID register."]
    #[inline(always)]
    #[must_use]
    pub fn receivedid(&mut self) -> ReceivedidW<LinidSpec> {
        ReceivedidW::new(self, 16)
    }
}
#[doc = "The LINID register contains the identification fields for LIN communication. NOTE: For software compatibility with future LIN modules, the HGEN CTRL bit must be set to 1, the RX ID MASK field must be set to FFh, and the TX ID MASK field must be set to FFh.\n\nYou can [`read`](crate::Reg::read) this register and get [`linid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinidSpec;
impl crate::RegisterSpec for LinidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linid::R`](R) reader structure"]
impl crate::Readable for LinidSpec {}
#[doc = "`write(|w| ..)` method takes [`linid::W`](W) writer structure"]
impl crate::Writable for LinidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINID to value 0"]
impl crate::Resettable for LinidSpec {
    const RESET_VALUE: u32 = 0;
}
